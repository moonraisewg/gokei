"use client"

import { Wallet, Fingerprint, Key } from "lucide-react"
import { useNavigate } from 'react-router-dom'

export default function CreateWallet() {
  const navigate = useNavigate();

  const handleCreateWallet = () => {
    // TODO: Thêm logic tạo ví ở đây
    navigate('/dashboard', { state: { fromCreate: true } });
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 flex items-center space-x-2">
        <button 
          onClick={() => navigate('/onboarding')}
          className="hover:bg-[#2A2A2A] p-2 rounded-lg"
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <Wallet className="w-5 h-5 text-white" />
        <h1 className="text-white text-lg">Create Wallet</h1>
      </div>

      {/* Content */}
      <div className="flex-1 px-6 pb-6 flex flex-col items-center justify-between">
        <div className="flex flex-col items-center">
          {/* Fingerprint Icon */}
          <div className="w-20 h-20 bg-[#2A2A2A] rounded-full flex items-center justify-center mb-8">
            <Fingerprint className="w-12 h-12 text-white" />
          </div>

          {/* Text */}
          <h2 className="text-white text-2xl font-medium text-center mb-2">Secure Your Wallet</h2>
          <p className="text-white text-center text-sm mb-8 max-w-[280px]">
            Use biometric authentication to create and protect your Solana wallet
          </p>
        </div>

        <div className="w-full">
          {/* Authentication Options */}
          <div className="w-full space-y-3 mb-8">
            <button 
              className="w-full bg-white text-black rounded-lg py-3 flex items-center justify-center space-x-2 hover:bg-gray-100"
              onClick={handleCreateWallet}
            >
              <Fingerprint className="w-5 h-5" />
              <span>Register with Biometrics</span>
            </button>

            <button 
              className="w-full bg-[#2A2A2A] text-white rounded-lg py-3 flex items-center justify-center space-x-2 hover:bg-[#333333]"
              onClick={handleCreateWallet}
            >
              <Key className="w-5 h-5" />
              <span>Use Security Key</span>
            </button>
          </div>

          {/* Footer */}
          <div className="text-white text-sm text-center">Powered by WebAuthn</div>
        </div>
      </div>
    </div>
  )
}

