"use client"

import { X, QrCode, Copy } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

interface WalletItem {
  icon: string
  name: string
  address: string
  color: string
}

export default function ReceiveTokens() {
  const navigate = useNavigate();

  const wallets: WalletItem[] = [
    {
      icon: "◎",
      name: "Solana",
      address: "AoNF...SbfW",
      color: "text-purple-500"
    },
    {
      icon: "Ξ",
      name: "Ethereum",
      address: "0x4B6C...c58b",
      color: "text-blue-500"
    },
    {
      icon: "⊙",
      name: "Base",
      address: "0x4B6C...c58b",
      color: "text-blue-400"
    },
    {
      icon: "⬡",
      name: "Polygon",
      address: "0x4B6C...c58b",
      color: "text-purple-400"
    },
    {
      icon: "₿",
      name: "Bitcoin",
      address: "bc1q...p5x0",
      color: "text-orange-500"
    },
  ]

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <QrCode className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Receive Tokens</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-hide p-4">
        {/* Wallet List */}
        <div className="space-y-2">
          {wallets.map((wallet, index) => (
            <div 
              key={index} 
              className="bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-3">
                  <div className="w-10 h-10 bg-black rounded-lg flex items-center justify-center">
                    <span className={`text-lg ${wallet.color}`}>{wallet.icon}</span>
                  </div>
                  <div>
                    <div className="text-white font-medium">{wallet.name}</div>
                    <div className="text-sm text-gray-400">{wallet.address}</div>
                  </div>
                </div>
                <div className="flex space-x-2">
                  <button className="p-2 hover:bg-[#404040] rounded-lg transition-colors">
                    <QrCode className="w-5 h-5 text-gray-400 hover:text-white transition-colors" />
                  </button>
                  <button className="p-2 hover:bg-[#404040] rounded-lg transition-colors">
                    <Copy className="w-5 h-5 text-gray-400 hover:text-white transition-colors" />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button 
          onClick={() => navigate('/dashboard')}
          className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  )
}

