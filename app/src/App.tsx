import Onboarding from "./pages/Onboarding";
import CreateWallet from "./pages/create-wallet";
import WalletDashboard from "./pages/wallet-dashboard";
import WalletAuth from "./pages/wallet-auth";
import SendTokens from "./pages/send-tokens";
import ReceiveTokens from "./pages/receive-tokens";
import TransactionHistory from "./pages/transaction-history";
import ConfirmTransaction from "./pages/confirm-transaction";
import CreateMultisigWallet from "./pages/create-multisig-wallet";
import MultiSignatureApproval from "./pages/multi-signature-approval";
import BuyToken from "./pages/buy-token";
import SwapToken from "./pages/swap-token";
import Setting from "./pages/setting";
import MultisigList from "./pages/multisig-list";
import CreateProposal from "./pages/create-proposal";
import { BrowserRouter as Router, Routes, Route, Navigate, useLocation } from "react-router-dom";

const App = () => {
  return (
    <Router>
      <div className="min-h-screen bg-black flex items-center justify-center">
        <div className="w-[400px] h-[600px] bg-black rounded-lg overflow-hidden">
          <Routes>
            <Route path="/" element={<CheckFirstLogin />} />
            <Route path="/onboarding" element={<Onboarding />} />
            <Route path="/create" element={<CreateWallet />} />
            <Route path="/dashboard" element={<WalletDashboard />} />
            <Route path="/auth" element={<WalletAuth />} />
            <Route path="/send" element={<SendTokens />} />
            <Route path="/receive" element={<ReceiveTokens />} />
            <Route path="/history" element={<TransactionHistory />} />
            <Route path="/confirm-transaction" element={<ConfirmTransaction />} />
            <Route path="/create-multisig" element={<CreateMultisigWallet />} />
            <Route path="/multisig-approval" element={<MultiSignatureApproval />} />
            <Route path="/settings" element={<Setting />} />
            <Route path="/buy" element={<BuyToken />} />
            <Route path="/swap" element={<SwapToken />} />
            <Route path="/multisig-list" element={<MultisigList />} />
            <Route path="/create-proposal" element={<CreateProposal />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

// Component kiểm tra lần đầu hoặc yêu cầu login
const CheckFirstLogin = () => {
  const location = useLocation();
  const hasWallet = localStorage.getItem("hasWallet");

  if (location.state?.fromCreate || location.state?.fromImport) {
    localStorage.setItem("hasWallet", "true");
    return <Navigate to="/dashboard" replace />;
  }

  if (hasWallet === "true") {
    return <Navigate to="/auth" replace />;
  }

  return <Navigate to="/onboarding" replace />;
};

export default App;