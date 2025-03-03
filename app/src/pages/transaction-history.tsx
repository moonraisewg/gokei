"use client"

import { ArrowUp, ArrowDown, RefreshCcw, Users, X, Clock } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function TransactionHistory() {
  const navigate = useNavigate();
  const tabs = ["All", "Sent", "Received", "Swapped"]

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Clock className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Transaction History</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-[#2A2A2A] px-4">
        {tabs.map((tab) => (
          <button
            key={tab}
            className={`px-4 py-3 text-sm border-b-2 transition-colors ${
              tab === "All" 
                ? "text-white border-white" 
                : "text-gray-400 border-transparent hover:text-white hover:border-[#2A2A2A]"
            }`}
          >
            {tab}
          </button>
        ))}
      </div>

      {/* Transactions List */}
      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-2">
        {/* Sent Transaction */}
        <div className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors">
          <div className="flex items-center justify-between">
            <div className="flex items-start space-x-3">
              <div className="p-2 bg-[#333333] rounded-lg">
                <ArrowUp className="w-5 h-5 text-red-500" />
              </div>
              <div>
                <div className="text-white font-medium">Sent ETH</div>
                <div className="text-sm text-gray-400">To: 0x1234...5678</div>
              </div>
            </div>
            <div className="text-right">
              <div className="text-red-500 font-medium">-0.5 ETH</div>
              <div className="text-sm text-gray-400 flex items-center justify-end">
                <span className="w-2 h-2 bg-green-500 rounded-full mr-1"></span>
                Completed
              </div>
            </div>
          </div>
        </div>

        {/* Received Transaction */}
        <div className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors">
          <div className="flex items-center justify-between">
            <div className="flex items-start space-x-3">
              <div className="p-2 bg-[#333333] rounded-lg">
                <ArrowDown className="w-5 h-5 text-green-500" />
              </div>
              <div>
                <div className="text-white font-medium">Received USDT</div>
                <div className="text-sm text-gray-400">From: 0x8765...4321</div>
              </div>
            </div>
            <div className="text-right">
              <div className="text-green-500 font-medium">+100 USDT</div>
              <div className="text-sm text-gray-400 flex items-center justify-end">
                <span className="w-2 h-2 bg-yellow-500 rounded-full mr-1"></span>
                Pending
              </div>
            </div>
          </div>
        </div>

        {/* Swapped Transaction */}
        <div className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors">
          <div className="flex items-center justify-between">
            <div className="flex items-start space-x-3">
              <div className="p-2 bg-[#333333] rounded-lg">
                <RefreshCcw className="w-5 h-5 text-blue-500" />
              </div>
              <div>
                <div className="text-white font-medium">Swapped ETH/USDC</div>
                <div className="text-sm text-gray-400">0.1 ETH → 180 USDC</div>
              </div>
            </div>
            <div className="text-right">
              <div className="text-white font-medium">180 USDC</div>
              <div className="text-sm text-gray-400 flex items-center justify-end">
                <span className="w-2 h-2 bg-red-500 rounded-full mr-1"></span>
                Failed
              </div>
            </div>
          </div>
        </div>

        {/* MultiSig Transaction */}
        <div className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors">
          <div className="flex items-center justify-between">
            <div className="flex items-start space-x-3">
              <div className="p-2 bg-[#333333] rounded-lg">
                <Users className="w-5 h-5 text-purple-500" />
              </div>
              <div>
                <div className="text-white font-medium">MultiSig Approval</div>
                <div className="text-sm text-gray-400">2/3 Signatures</div>
              </div>
            </div>
            <div className="text-right">
              <div className="text-white font-medium">1.5 ETH</div>
              <div className="text-sm text-gray-400 flex items-center justify-end">
                <span className="w-2 h-2 bg-yellow-500 rounded-full mr-1"></span>
                Pending
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="flex justify-between items-center px-4 py-3 border-t border-[#2A2A2A] text-sm">
        <div className="text-gray-400">Last updated: 2 min ago</div>
        <button className="flex items-center space-x-1 text-gray-400 hover:text-white transition-colors">
          <RefreshCcw className="w-4 h-4" />
          <span>Refresh</span>
        </button>
      </div>
    </div>
  )
}

