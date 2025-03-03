"use client"

import { Fingerprint, X } from "lucide-react"
import { useNavigate } from 'react-router-dom'

export default function ConfirmTransaction() {
  const navigate = useNavigate();

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Fingerprint className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Confirm Transaction</h2>
        </div>
        <button 
          onClick={() => navigate(-1)}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      <div className="flex-1 flex flex-col items-center justify-center p-4 space-y-6">
        {/* Fingerprint Icon */}
        <div className="p-8 bg-[#2A2A2A] rounded-full">
          <Fingerprint className="w-16 h-16 text-white" />
        </div>

        {/* Verification Message */}
        <div className="text-center space-y-1">
          <p className="text-white font-medium">Please verify your identity</p>
          <p className="text-sm text-gray-400">Use biometric authentication to approve</p>
        </div>

        {/* Action Buttons */}
        <div className="w-full space-y-3">
          <button 
            onClick={() => navigate('/dashboard')}
            className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors flex items-center justify-center space-x-2"
          >
            <Fingerprint className="w-5 h-5" />
          </button>
          <button 
            onClick={() => navigate(-1)}
            className="w-full text-gray-400 hover:text-white py-3 rounded-lg transition-colors"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  )
}

