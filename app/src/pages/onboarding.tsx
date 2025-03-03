"use client"

import { Wallet, Download, Shield } from "lucide-react"
import { useNavigate } from 'react-router-dom'

export default function Onboarding() {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-black">
      <div className="w-[400px] max-w-full px-6 py-8 flex flex-col items-center justify-center">
        <div className="flex flex-col items-center justify-center w-full">
          {/* Wallet Icon */}
          <div className="bg-[#2A2A2A] p-4 rounded-2xl mb-6">
            <Wallet className="w-8 h-8 text-white" />
          </div>

          {/* Welcome Text */}
          <h1 className="text-2xl font-medium text-white mb-2">Welcome to SolWallet</h1>
          <p className="text-white text-sm mb-10">Your secure gateway to the Solana ecosystem</p>

          {/* Buttons */}
          <div className="w-full space-y-4 mb-10">
            <button
              className="w-full bg-[#2A2A2A] hover:bg-[#333333] text-white py-3 px-4 rounded-lg flex items-center justify-center transition-colors"
              onClick={() => navigate('/create', { state: { fromCreate: true } })}
            >
              <span className="mr-2 text-lg">+</span> Create a New Wallet
            </button>

            <button
              className="w-full bg-[#1E1E1E] hover:bg-[#252525] text-white py-3 px-4 rounded-lg flex items-center justify-center transition-colors"
              onClick={() => navigate('/create', { state: { fromImport: true } })}
            >
              <Download className="w-5 h-5 mr-2" /> Import Existing Wallet
            </button>
          </div>

          {/* Footer */}
          <div className="flex flex-col items-center text-xs text-white space-y-2 mt-auto">
            <div className="flex items-center">
              <Shield className="w-4 h-4 mr-2" /> Bank-grade security
            </div>
            <div className="flex items-center">
              <span className="text-xs">Powered by Solana</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

