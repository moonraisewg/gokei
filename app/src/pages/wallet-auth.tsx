"use client"

import { X, HelpCircle, Fingerprint, Key } from "lucide-react"

export default function WalletAuth() {
  return (
    <div className="w-full max-w-md bg-black rounded-xl shadow-lg">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b">
        <div className="flex items-center space-x-2">
          <div className="w-6 h-6 rounded-full border border-white-200 flex items-center justify-center">
            <HelpCircle className="w-4 h-4 text-white" />
          </div>
          <span className="font-medium">Solana Wallet</span>
        </div>
        <button className="text-white hover:text-white">
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Content */}
      <div className="p-8 flex flex-col items-center">
        {/* Fingerprint Icon */}
        <div className="w-20 h-20 bg-white-100 rounded-full flex items-center justify-center mb-6">
          <Fingerprint className="w-12 h-12 text-white" />
        </div>

        {/* Text */}
        <h1 className="text-2xl font-semibold mb-2">Verify Your Identity</h1>
        <p className="text-white text-center mb-8">Use biometric authentication to access your wallet</p>

        {/* Authentication Options */}
        <div className="w-full space-y-3">
          <button className="w-full bg-black text-white rounded-lg py-3 flex items-center justify-center space-x-2 hover:bg-black/90">
            <Fingerprint className="w-5 h-5" />
            <span>Authenticate with Biometrics</span>
          </button>

          <button className="w-full bg-black border border-white-200 text-white rounded-lg py-3 flex items-center justify-center space-x-2 hover:bg-white-50">
            <Key className="w-5 h-5" />
            <span>Use Password</span>
          </button>
        </div>

        {/* Help Link */}
        <button className="mt-8 text-white hover:text-white text-sm">Need help?</button>
      </div>
    </div>
  )
}

