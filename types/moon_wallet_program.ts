/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/moon_wallet_program.json`.
 */
export type MoonWalletProgram = {
  "address": "DeN1rBfabZezHPvrq9q7BbzUbZkrjnHE1kQDrPK8kWQ3",
  "metadata": {
    "name": "moonWalletProgram",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addGuardian",
      "discriminator": [
        167,
        189,
        170,
        27,
        74,
        240,
        201,
        241
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "guardianId",
          "type": "u64"
        },
        {
          "name": "guardianName",
          "type": "string"
        },
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "isOwner",
          "type": "bool"
        },
        {
          "name": "webauthnPubkey",
          "type": {
            "option": {
              "array": [
                "u8",
                33
              ]
            }
          }
        }
      ]
    },
    {
      "name": "initializeMultisig",
      "discriminator": [
        220,
        130,
        117,
        21,
        27,
        227,
        78,
        213
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "feePayer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "threshold",
          "type": "u8"
        },
        {
          "name": "credentialId",
          "type": "string"
        }
      ]
    },
    {
      "name": "recoverAccessByGuardian",
      "discriminator": [
        210,
        31,
        244,
        215,
        121,
        93,
        165,
        99
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "oldGuardian",
          "docs": [
            "Guardian cũ (không phải owner nữa)"
          ],
          "writable": true
        },
        {
          "name": "oldGuardianPubkey"
        },
        {
          "name": "newGuardian",
          "docs": [
            "Guardian mới (sẽ trở thành owner)"
          ],
          "writable": true
        },
        {
          "name": "newGuardianPubkey"
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "oldGuardianId",
          "type": "u64"
        },
        {
          "name": "newGuardianId",
          "type": "u64"
        },
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "newWebauthnPubkey",
          "type": {
            "array": [
              "u8",
              33
            ]
          }
        }
      ]
    },
    {
      "name": "removeGuardian",
      "discriminator": [
        72,
        117,
        160,
        244,
        155,
        185,
        71,
        18
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "ownerGuardian",
          "docs": [
            "Yêu cầu phải được ký bởi một guardian có quyền owner"
          ]
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "guardianId",
          "type": "u64"
        },
        {
          "name": "ownerGuardianId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateGuardianStatus",
      "discriminator": [
        17,
        169,
        132,
        234,
        235,
        231,
        211,
        79
      ],
      "accounts": [
        {
          "name": "multisig"
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "ownerGuardian",
          "docs": [
            "Tài khoản guardian của người gọi, phải là owner"
          ]
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "guardianId",
          "type": "u64"
        },
        {
          "name": "ownerGuardianId",
          "type": "u64"
        },
        {
          "name": "isActive",
          "type": "bool"
        }
      ]
    },
    {
      "name": "verifyAndExecute",
      "discriminator": [
        37,
        165,
        237,
        189,
        225,
        188,
        58,
        41
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian",
          "docs": [
            "Tìm guardian owner có webauthn_pubkey mà chúng ta cần xác thực"
          ]
        },
        {
          "name": "clock"
        },
        {
          "name": "instructionSysvar"
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "destination",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "action",
          "type": "string"
        },
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "actionParams"
            }
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "timestamp",
          "type": "i64"
        },
        {
          "name": "message",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "guardian",
      "discriminator": [
        57,
        234,
        122,
        214,
        12,
        246,
        9,
        45
      ]
    },
    {
      "name": "multiSigWallet",
      "discriminator": [
        93,
        17,
        107,
        133,
        10,
        77,
        189,
        238
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidOperation",
      "msg": "Không có quyền hoặc dữ liệu không hợp lệ"
    },
    {
      "code": 6001,
      "name": "limitExceeded",
      "msg": "Giới hạn đã đạt tối đa"
    },
    {
      "code": 6002,
      "name": "guardianError",
      "msg": "Guardian không hợp lệ"
    },
    {
      "code": 6003,
      "name": "invalidConfig",
      "msg": "Cấu hình không hợp lệ"
    },
    {
      "code": 6004,
      "name": "invalidRecovery",
      "msg": "Recovery không hợp lệ"
    },
    {
      "code": 6005,
      "name": "invalidThreshold",
      "msg": "Ngưỡng không hợp lệ"
    },
    {
      "code": 6006,
      "name": "webAuthnNotConfigured",
      "msg": "WebAuthn chưa được cấu hình"
    },
    {
      "code": 6007,
      "name": "nameTooLong",
      "msg": "Tên ví không được vượt quá 32 ký tự"
    },
    {
      "code": 6008,
      "name": "invalidRecoveryKey",
      "msg": "Recovery key không hợp lệ"
    },
    {
      "code": 6009,
      "name": "noGuardians",
      "msg": "Không có guardian nào để xóa"
    },
    {
      "code": 6010,
      "name": "invalidNonce",
      "msg": "Nonce không hợp lệ"
    },
    {
      "code": 6011,
      "name": "futureTimestamp",
      "msg": "Timestamp thuộc về tương lai"
    },
    {
      "code": 6012,
      "name": "outdatedTimestamp",
      "msg": "Timestamp quá cũ"
    },
    {
      "code": 6013,
      "name": "expiredTimestamp",
      "msg": "Timestamp đã hết hạn"
    },
    {
      "code": 6014,
      "name": "instructionMissing",
      "msg": "Instruction xác thực chữ ký bị thiếu"
    },
    {
      "code": 6015,
      "name": "invalidSignatureVerification",
      "msg": "Xác thực chữ ký không hợp lệ"
    },
    {
      "code": 6016,
      "name": "publicKeyMismatch",
      "msg": "Public key không khớp với wallet"
    },
    {
      "code": 6017,
      "name": "messageMismatch",
      "msg": "Message không khớp"
    },
    {
      "code": 6018,
      "name": "invalidInstructionData",
      "msg": "Dữ liệu instruction không hợp lệ"
    },
    {
      "code": 6019,
      "name": "invalidSignatureCount",
      "msg": "Số lượng chữ ký không hợp lệ"
    },
    {
      "code": 6020,
      "name": "unsupportedAction",
      "msg": "Hành động không được hỗ trợ"
    },
    {
      "code": 6021,
      "name": "invalidGuardian",
      "msg": "Guardian không hợp lệ hoặc không được tìm thấy"
    },
    {
      "code": 6022,
      "name": "inactiveGuardian",
      "msg": "Guardian đang không hoạt động"
    }
  ],
  "types": [
    {
      "name": "actionParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "destination",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "tokenMint",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "guardian",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "guardianId",
            "type": "u64"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "recoveryHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "isOwner",
            "type": "bool"
          },
          {
            "name": "webauthnPubkey",
            "type": {
              "option": {
                "array": [
                  "u8",
                  33
                ]
              }
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multiSigWallet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "guardianCount",
            "type": "u8"
          },
          {
            "name": "recoveryNonce",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "transactionNonce",
            "type": "u64"
          },
          {
            "name": "lastTransactionTimestamp",
            "type": "i64"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "credentialId",
            "type": "string"
          }
        ]
      }
    }
  ]
};
