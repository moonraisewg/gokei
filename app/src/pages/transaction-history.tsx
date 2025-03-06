"use client"

import { useState } from "react"
import { ArrowUp, ArrowDown, RefreshCcw, Users, X, Clock, ArrowLeft } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

const transactions = [
  { id: 1, type: "Sent", icon: <ArrowUp className="w-5 h-5 text-red-500" />, title: "Sent ETH", details: "To: 0x1234...5678", amount: "-0.5 ETH", status: "Completed", statusColor: "green-500" },
  { id: 2, type: "Received", icon: <ArrowDown className="w-5 h-5 text-green-500" />, title: "Received USDT", details: "From: 0x8765...4321", amount: "+100 USDT", status: "Pending", statusColor: "yellow-500" },
  { id: 3, type: "Swapped", icon: <RefreshCcw className="w-5 h-5 text-blue-500" />, title: "Swapped ETH/USDC", details: "0.1 ETH → 180 USDC", amount: "180 USDC", status: "Failed", statusColor: "red-500" },
  { id: 4, type: "MultiSig", icon: <Users className="w-5 h-5 text-purple-500" />, title: "MultiSig Approval", details: "2/3 Signatures", amount: "1.5 ETH", status: "Pending", statusColor: "yellow-500" },
]

export default function TransactionHistory() {
  const navigate = useNavigate();
  const tabs = ["All", "Sent", "Received", "Swapped", "MultiSig"]
  const [activeTab, setActiveTab] = useState("All")

  // Lọc giao dịch theo tab
  const filteredTransactions = activeTab === "All" ? transactions : transactions.filter(tx => tx.type === activeTab)

  return (
      <div className="h-full flex flex-col bg-black">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
          <div className="flex items-center space-x-2">
            <button onClick={() => navigate('/dashboard')} className="hover:bg-[#2A2A2A] p-2 rounded-lg">
              <ArrowLeft className="h-5 w-5 text-white"/>
            </button>
            <Clock className="w-5 h-5 text-white"/>
            <h2 className="text-lg font-medium text-white">Transaction History</h2>
          </div>
          <button onClick={() => navigate('/dashboard')} className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg">
            <X className="w-5 h-5"/>
          </button>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-[#2A2A2A] px-4">
          {tabs.map((tab) => (
              <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-4 py-3 text-sm border-b-2 transition-colors ${
                      activeTab === tab
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
          {filteredTransactions.length > 0 ? (
              filteredTransactions.map((tx) => (
                  <div key={tx.id} className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors">
                    <div className="flex items-center justify-between">
                      <div className="flex items-start space-x-3">
                        <div className="p-2 bg-[#333333] rounded-lg">{tx.icon}</div>
                        <div>
                          <div className="text-white font-medium">{tx.title}</div>
                          <div className="text-sm text-gray-400">{tx.details}</div>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className={`font-medium ${tx.type === "Sent" ? "text-red-500" : tx.type === "Received" ? "text-green-500" : "text-white"}`}>
                          {tx.amount}
                        </div>
                        <div className="text-sm text-gray-400 flex items-center justify-end">
                          <span className={`w-2 h-2 bg-${tx.statusColor} rounded-full mr-1`}></span>
                          {tx.status}
                        </div>
                      </div>
                    </div>
                  </div>
              ))
          ) : (
              <div className="text-center text-gray-400">No transactions found</div>
          )}
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
