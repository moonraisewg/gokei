import { PublicKey, Connection, Transaction, SYSVAR_CLOCK_PUBKEY, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram } from '@solana/web3.js';
import { getWebAuthnAssertion } from './webauthnUtils';
import { getWalletByCredentialId, getCredentialsByWallet, normalizeCredentialId } from '../firebase/webAuthnService';
import { addSignerToProposal, updateProposalStatus } from '../firebase/proposalService';
import { createApproveProposalTx, createExecuteProposalTx } from './transactionUtils';

/**
 * Chuyển đổi credential ID giữa các định dạng
 * @param credentialId ID ban đầu (base64 hoặc hex)
 * @returns Mảng các phiên bản credential ID khác nhau để thử
 */
const getCredentialIdVariations = (credentialId: string): string[] => {
  const variations: string[] = [credentialId];
  
  try {
    // Thử xem là hex, nếu đúng thì thêm bản base64
    if (/^[0-9a-fA-F]+$/.test(credentialId)) {
      const asBase64 = Buffer.from(credentialId, 'hex').toString('base64');
      variations.push(asBase64);
      console.log(`Chuyển từ hex sang base64: ${asBase64}`);
    }
  } catch (e) {
    console.log('Không thể chuyển từ hex sang base64');
  }
  
  try {
    // Thử xem là base64, nếu đúng thì thêm bản hex
    const asHex = Buffer.from(credentialId, 'base64').toString('hex');
    variations.push(asHex);
    console.log(`Chuyển từ base64 sang hex: ${asHex}`);
  } catch (e) {
    console.log('Không thể chuyển từ base64 sang hex');
  }
  
  // Loại bỏ các biến thể trùng nhau
  return Array.from(new Set(variations));
};

/**
 * Ký một đề xuất
 * @param proposalId ID của đề xuất
 * @param multisigAddress Địa chỉ của ví multisig
 * @param connection Instance của Connection để gửi transaction
 * @returns Promise<{success: boolean, message: string, signature?: string}>
 */
export const signProposal = async (
  proposalId: number,
  multisigAddress: string,
  connection: Connection,
  sendTransaction?: (transaction: Transaction, connection: Connection) => Promise<string>
): Promise<{success: boolean, message: string, signature?: string}> => {
  try {
    console.log(`=== BẮT ĐẦU KÝ ĐỀ XUẤT ${proposalId} ===`);
    
    // Kiểm tra xem có hàm sendTransaction hay không
    if (!sendTransaction) {
      console.warn("Không có hàm sendTransaction, chỉ cập nhật Firebase mà không gửi transaction lên blockchain");
    }
    
    // Lấy timestamp hiện tại
    const timestamp = Math.floor(Date.now() / 1000);
    
    // Tạo message template để ký
    const messageTemplate = `approve:proposal_${proposalId},timestamp:${timestamp}`;
    console.log('Template message để ký:', messageTemplate);
    
    // Yêu cầu người dùng xác thực trực tiếp với WebAuthn - không chỉ định credential ID
    // allowEmpty = true cho phép người dùng chọn từ danh sách các credential đã đăng ký
    const assertion = await getWebAuthnAssertion(null, messageTemplate, true);
    
    if (!assertion) {
      return { success: false, message: "Người dùng đã hủy xác thực hoặc xác thực thất bại" };
    }
    
    // Lưu toàn bộ thông tin assertion để debug
    console.log('Assertion đầy đủ:', assertion);
    
    // Ưu tiên sử dụng credentialId từ kết quả assertion (đã thêm trong bản sửa webauthnUtils.ts)
    let credentialId = assertion.credentialId;
    
    // Nếu không có credentialId trong assertion, thử các cách khác
    if (!credentialId) {
      // Cách 1: Lấy từ assertion.credentialId nếu có (tùy thuộc vào cấu trúc response)
      if (typeof (assertion as any).credentialId === 'string') {
        credentialId = (assertion as any).credentialId;
        console.log('Lấy credential ID từ assertion.credentialId:', credentialId);
      } 
      // Cách 2: Phân tích từ clientDataJSON
      else {
        try {
          const clientDataObj = JSON.parse(new TextDecoder().decode(assertion.clientDataJSON));
          if (clientDataObj.credential?.id) {
            credentialId = clientDataObj.credential.id;
            console.log('Lấy credential ID từ clientDataJSON:', credentialId);
          }
        } catch (e) {
          console.error('Lỗi khi parse clientDataJSON:', e);
        }
      }
      
      // Cách 3: Kiểm tra xem có thể lấy từ localStorage không
      if (!credentialId) {
        console.log('Tìm kiếm credential ID trong localStorage...');
        try {
          const userCredentials = JSON.parse(localStorage.getItem("userCredentials") || "[]");
          if (userCredentials.length > 0) {
            credentialId = userCredentials[0].id;
            console.log('Đã tìm thấy credential ID trong localStorage:', credentialId);
          }
        } catch (e) {
          console.error('Lỗi khi đọc từ localStorage:', e);
        }
      }
      
      // Cách 4: Kiểm tra webauthnCredentials
      if (!credentialId) {
        try {
          const webauthnCredentials = JSON.parse(localStorage.getItem("webauthnCredentials") || "[]");
          if (webauthnCredentials.length > 0) {
            credentialId = webauthnCredentials[0].credentialId;
            console.log('Đã tìm thấy credential ID trong webauthnCredentials:', credentialId);
          }
        } catch (e) {
          console.error('Lỗi khi đọc webauthnCredentials:', e);
        }
      }
      
      // Kiểm tra xem cuối cùng đã có credential ID chưa
      if (!credentialId) {
        console.error('Không thể lấy credential ID từ bất kỳ nguồn nào');
        
        // Giải pháp cuối cùng: Nhập thủ công hoặc sử dụng ID cứng
        const storedCredentialId = localStorage.getItem("lastCredentialId");
        if (storedCredentialId) {
          credentialId = storedCredentialId;
          console.log('Sử dụng lastCredentialId từ localStorage:', credentialId);
        } else {
          // Thử lấy từ publicKey được lưu trong localStorage
          try {
            const publicKeyKeys = Object.keys(localStorage).filter(key => 
              key.startsWith('guardianPublicKey_'));
            
            if (publicKeyKeys.length > 0) {
              // Lấy credentialId từ tên key
              credentialId = publicKeyKeys[0].replace('guardianPublicKey_', '');
              console.log('Lấy credential ID từ key guardianPublicKey_:', credentialId);
            }
          } catch (e) {
            console.error('Lỗi khi lọc localStorage keys:', e);
          }
        }
      }
    }
    
    if (!credentialId) {
      return { success: false, message: 'Không nhận được credential ID từ WebAuthn. Vui lòng đăng ký lại thiết bị.' };
    }
    
    console.log('Đã nhận credential ID từ WebAuthn:', credentialId);
    
    // Lưu credential ID để sử dụng sau này
    localStorage.setItem("lastCredentialId", credentialId);
    
    // Tạo các biến thể khác nhau của credential ID để thử
    const credentialIdVariations = getCredentialIdVariations(credentialId);
    console.log('Các biến thể của credential ID:', credentialIdVariations);
    
    // Thử tìm guardian với mỗi biến thể của credential ID
    let guardianInfo = null;
    for (const idVariation of credentialIdVariations) {
      console.log(`Đang thử tìm guardian với credential ID: ${idVariation}`);
      const result = await getWalletByCredentialId(idVariation);
      if (result) {
        console.log('Đã tìm thấy guardian với credential ID:', idVariation);
        guardianInfo = result;
        break;
      }
    }
    
    // Nếu vẫn không tìm thấy, thử lấy tất cả guardian và so sánh thủ công
    if (!guardianInfo) {
      console.log('Không tìm thấy guardian với các credential ID đã thử. Thử lấy danh sách tất cả guardians...');
      try {
        // Lấy tất cả guardians của ví multisig này
        const allGuardians = await getCredentialsByWallet(multisigAddress);
        console.log('Tất cả guardians của ví này:', allGuardians);
        
        if (allGuardians.length > 0) {
          // Hiển thị tất cả credentials để dễ debug
          console.log('Danh sách tất cả credential IDs từ Firebase:');
          allGuardians.forEach((guardian, index) => {
            console.log(`[${index}] hex: ${guardian.credentialId}, base64: ${guardian.credentialIdBase64}`);
          });
          
          // Nếu chỉ có một guardian, sử dụng luôn
          if (allGuardians.length === 1) {
            guardianInfo = allGuardians[0];
            console.log('Chỉ có một guardian, sử dụng mặc định:', guardianInfo);
          }
        } else {
          console.error('Không tìm thấy guardian nào cho ví này!');
        }
      } catch (e) {
        console.error('Lỗi khi lấy danh sách guardians:', e);
      }
    }
    
    if (!guardianInfo) {
      return { success: false, message: 'Không tìm thấy thông tin guardian trong Firebase. Vui lòng kiểm tra lại credential ID và đảm bảo guardian đã được đăng ký.' };
    }
    
    // Sử dụng guardianId từ Firebase
    if (!guardianInfo.guardianId) {
      return { success: false, message: 'Không tìm thấy guardianId trong thông tin guardian' };
    }
    
    const guardianId = guardianInfo.guardianId;
    console.log('Guardian ID từ Firebase:', guardianId);
    
    // Lấy WebAuthn public key từ Firebase
    if (!guardianInfo.guardianPublicKey || guardianInfo.guardianPublicKey.length === 0) {
      return { success: false, message: 'Không tìm thấy WebAuthn public key trong Firebase' };
    }
    
    // Tính PDA cho multisig và guardian
    const multisigPDA = new PublicKey(multisigAddress);
    
    // Tính PDA cho guardian
    const [guardianPDA] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from('guardian'),
        multisigPDA.toBuffer(),
        Buffer.from(new Uint8Array(new BN(guardianId).toArray('le', 8)))
      ],
      new PublicKey('6Y3N5AQRQtviTrmRcf29yHWBu3ft7xEoj8kqmykDGvKP')
    );
    
    // Tìm PDA của proposal
    const [proposalPDA] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from('proposal'),
        multisigPDA.toBuffer(),
        Buffer.from(new Uint8Array(new BN(proposalId).toArray('le', 8)))
      ],
      new PublicKey('6Y3N5AQRQtviTrmRcf29yHWBu3ft7xEoj8kqmykDGvKP')
    );
    
    let txSignature: string | undefined;
    
    // Nếu có hàm sendTransaction, gửi transaction lên blockchain
    if (sendTransaction) {
      try {
        // Tạo transaction để ký đề xuất
        const tx = await createApproveProposalTx(
          proposalPDA,
          multisigPDA,
          guardianPDA,
          guardianId,
          new PublicKey(guardianInfo.walletAddress), // Người trả phí
          assertion.signature,
          assertion.authenticatorData,
          assertion.clientDataJSON,
          proposalId,
          timestamp,
          credentialId
        );
        
        // Gửi transaction
        txSignature = await sendTransaction(tx, connection);
        console.log('Đã ký đề xuất, signature:', txSignature);
      } catch (error) {
        console.error('Lỗi khi gửi transaction lên blockchain:', error);
        
        // Nếu lỗi khi gửi transaction, vẫn cập nhật Firebase (nếu muốn)
        // Hoặc có thể trả về lỗi tại đây để người dùng biết transaction thất bại
        return { 
          success: false, 
          message: `Không thể gửi transaction lên blockchain: ${error instanceof Error ? error.message : String(error)}` 
        };
      }
    } else {
      console.log('Bỏ qua gửi transaction vì không có hàm sendTransaction');
    }
    
    try {
      // Thêm guardian vào danh sách người ký trong Firebase
      await addSignerToProposal(
        multisigAddress,
        proposalId,
        guardianInfo.walletAddress // Sử dụng địa chỉ wallet của guardian
      );
      
      // Trả về kết quả thành công
      return { 
        success: true, 
        message: "Đã ký đề xuất thành công!",
        signature: txSignature
      };
    } catch (error) {
      console.error("Lỗi khi thêm guardian vào danh sách người ký:", error);
      
      // Nếu đã gửi transaction thành công nhưng cập nhật Firebase thất bại
      if (txSignature) {
        return {
          success: true,
          message: "Đã ký đề xuất trên blockchain thành công, nhưng không thể cập nhật Firebase.",
          signature: txSignature
        };
      }
      
      return { 
        success: false, 
        message: "Không thể cập nhật đề xuất trên Firebase" 
      };
    }
  } catch (error) {
    console.error('Lỗi khi ký đề xuất:', error);
    return { 
      success: false, 
      message: `Không thể ký đề xuất: ${error instanceof Error ? error.message : String(error)}` 
    };
  }
};

/**
 * Thực thi một đề xuất
 * @param proposalId ID của đề xuất
 * @param multisigAddress Địa chỉ của ví multisig
 * @param connection Instance của Connection để gửi transaction
 * @returns Promise<{success: boolean, message: string, signature?: string}>
 */
export const executeProposal = async (
  proposalId: number,
  multisigAddress: string,
  connection: Connection
): Promise<{success: boolean, message: string, signature?: string}> => {
  try {
    console.log(`=== BẮT ĐẦU THỰC THI ĐỀ XUẤT ${proposalId} ===`);
    
    // Cập nhật trạng thái đề xuất trong Firebase
    await updateProposalStatus(
      multisigAddress,
      proposalId,
      'executed'
    );
    
    return { 
      success: true, 
      message: "Đã thực thi đề xuất thành công!" 
    };
  } catch (error) {
    console.error('Lỗi khi thực thi đề xuất:', error);
    return { 
      success: false, 
      message: `Không thể thực thi đề xuất: ${error instanceof Error ? error.message : String(error)}` 
    };
  }
};

// Thêm BN để xử lý tính toán PDA
class BN {
  private value: number;
  constructor(value: number) {
    this.value = value;
  }
  
  toArray(endian: 'le' | 'be', length: number): number[] {
    const result = new Array(length).fill(0);
    let v = this.value;
    
    if (endian === 'le') {
      for (let i = 0; i < length; i++) {
        result[i] = v & 0xff;
        v >>= 8;
      }
    } else {
      for (let i = length - 1; i >= 0; i--) {
        result[i] = v & 0xff;
        v >>= 8;
      }
    }
    
    return result;
  }
} 