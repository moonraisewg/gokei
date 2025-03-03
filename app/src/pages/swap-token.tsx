"use client"

import { useState } from "react"
import { X, RefreshCcw, ChevronDown } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function SwapToken() {
  const navigate = useNavigate();
  const [fromAmount, setFromAmount] = useState("");
  const [toAmount, setToAmount] = useState("");

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <RefreshCcw className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Swap Tokens</h2>
        </div>
        <div className="flex items-center space-x-2">
          <button 
            onClick={() => navigate('/dashboard')}
            className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-hide px-4 py-3 space-y-3">
        {/* From Token */}
        <div className="p-4 bg-[#2A2A2A] rounded-lg space-y-2 hover:bg-[#333333] transition-colors">
          <div className="flex justify-between text-sm text-gray-400">
            <span>From</span>
            <span>Balance: 125.45 SOL</span>
          </div>
          <div className="flex items-center space-x-3">
            <button className="flex items-center space-x-2 bg-[#333333] px-3 py-2 rounded-lg hover:bg-[#404040] transition-colors">
              <div className="w-6 h-6 rounded-full bg-black flex items-center justify-center">
                <span className="text-purple-500">◎</span>
              </div>
              <span className="text-white">SOL</span>
              <ChevronDown className="w-4 h-4 text-white" />
            </button>
            <div className="flex-1 min-w-0">
              <input
                type="text"
                placeholder="0.0"
                value={fromAmount}
                onChange={(e) => setFromAmount(e.target.value)}
                className="w-full bg-transparent text-white text-right text-2xl outline-none"
              />
            </div>
          </div>
          <div className="text-right text-sm text-gray-400">≈ $0.00</div>
        </div>

        {/* Swap Button */}
        <div className="flex justify-center -my-1.5 z-10">
          <button className="w-10 h-10 bg-[#2A2A2A] rounded-full flex items-center justify-center hover:bg-[#333333] transition-colors shadow-lg">
            <RefreshCcw className="w-5 h-5 text-blue-500" />
          </button>
        </div>

        {/* To Token */}
        <div className="p-4 bg-[#2A2A2A] rounded-lg space-y-2 hover:bg-[#333333] transition-colors">
          <div className="flex justify-between text-sm text-gray-400">
            <span>To</span>
            <span>Balance: 1,245 USDC</span>
          </div>
          <div className="flex items-center space-x-3">
            <button className="flex items-center space-x-2 bg-[#333333] px-3 py-2 rounded-lg hover:bg-[#404040] transition-colors">
              <div className="w-6 h-6 rounded-full bg-black flex items-center justify-center">
                <span className="text-blue-500">$</span>
              </div>
              <span className="text-white">USDC</span>
              <ChevronDown className="w-4 h-4 text-white" />
            </button>
            <div className="flex-1 min-w-0">
              <input
                type="text"
                placeholder="0.0"
                value={toAmount}
                onChange={(e) => setToAmount(e.target.value)}
                className="w-full bg-transparent text-white text-right text-2xl outline-none"
              />
            </div>
          </div>
          <div className="text-right text-sm text-gray-400">≈ $0.00</div>
        </div>

        {/* Exchange Rate & Fees */}
        <div className="p-4 bg-[#2A2A2A] rounded-lg space-y-2 text-sm hover:bg-[#333333] transition-colors">
          <div className="flex justify-between">
            <span className="text-gray-400">Exchange Rate</span>
            <span className="text-white">1 SOL = 18.5 USDC</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">Network Fee</span>
            <span className="text-white">0.000005 SOL</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">Price Impact</span>
            <span className="text-green-500">0.05%</span>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors">
          Review Swap
        </button>
      </div>
    </div>
  )
}

