"use client"

import { useState } from "react"
import { X, Copy, ChevronDown, Send } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function SendTokens() {
  const [amount, setAmount] = useState("")
  const [recipientAddress, setRecipientAddress] = useState("")
  const navigate = useNavigate();

  const handleSendTransaction = () => {
    navigate('/confirm-transaction');
  };

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Send className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Send Tokens</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-4">
        {/* Wallet Balance */}
        <div className="bg-[#2A2A2A] rounded-lg p-4 flex justify-between items-center">
          <span className="text-gray-400">Wallet Balance</span>
          <div className="flex items-center space-x-2">
            <div className="w-6 h-6 rounded-full bg-black flex items-center justify-center">
              <span className="text-white">◎</span>
            </div>
            <span className="text-white font-medium">125.45 SOL</span>
          </div>
        </div>

        {/* Select Token */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Select Token</label>
          <button className="w-full flex items-center justify-between p-3 bg-[#2A2A2A] rounded-lg hover:bg-[#333333] transition-colors">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 rounded-full bg-black flex items-center justify-center">
                <span className="text-white">◎</span>
              </div>
              <div className="text-left">
                <div className="text-white">Solana</div>
                <div className="text-sm text-gray-400">SOL</div>
              </div>
            </div>
            <ChevronDown className="w-5 h-5 text-white" />
          </button>
        </div>

        {/* Amount */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Amount</label>
          <div className="p-3 bg-[#2A2A2A] rounded-lg space-y-2">
            <div className="relative">
              <input
                type="text"
                placeholder="0.00"
                className="w-full bg-transparent text-white text-2xl outline-none"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
              />
              <button className="absolute right-0 top-1/2 transform -translate-y-1/2 text-white text-sm bg-[#333333] px-2 py-1 rounded hover:bg-[#404040] transition-colors">
                MAX
              </button>
            </div>
            <div className="text-sm text-gray-400">≈ $0.00 USD</div>
          </div>
        </div>

        {/* Recipient Address */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Recipient Address</label>
          <div className="flex">
            <div className="flex-1 relative">
              <input
                type="text"
                placeholder="Enter Solana address"
                className="w-full p-3 bg-[#2A2A2A] text-white rounded-lg outline-none hover:bg-[#333333] transition-colors"
                value={recipientAddress}
                onChange={(e) => setRecipientAddress(e.target.value)}
              />
              <button className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-white transition-colors">
                <Copy className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>

        {/* Network Fee */}
        <div className="p-3 bg-[#2A2A2A] rounded-lg space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-400">Network Fee</span>
            <span className="text-white">0.000005 SOL</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">Total Amount</span>
            <span className="text-white">{amount || "0.00"} SOL</span>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button 
          onClick={handleSendTransaction}
          className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors"
        >
          Send Transaction
        </button>
      </div>
    </div>
  )
}

