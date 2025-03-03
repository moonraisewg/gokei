"use client"

import { Home, Users, Clock, Settings, Send, QrCode, RefreshCcw, CreditCard } from "lucide-react"
import { useNavigate } from 'react-router-dom'

export default function WalletDashboard() {
  const navigate = useNavigate();

  const quickActions = [
    { icon: Send, label: "Send", path: "/send" },
    { icon: QrCode, label: "Receive", path: "/receive" },
    { icon: RefreshCcw, label: "Swap", path: "/swap" },
    { icon: CreditCard, label: "Buy", path: "/buy" },
  ];

  const navItems = [
    { icon: Home, label: "Home", path: "/dashboard", active: true },
    { icon: Users, label: "Multi-Sig", path: "/multisig-list" },
    { icon: Clock, label: "Activity", path: "/history" },
    { icon: Settings, label: "Settings", path: "/settings" },
  ];

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="px-5 pt-6 pb-4">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-xl font-medium text-white">My Wallet</h1>
          <div className="w-8 h-8 rounded-full bg-[#2A2A2A] flex items-center justify-center">
            <span className="text-xl">👨</span>
          </div>
        </div>

        {/* Balance */}
        <div className="mb-6">
          <div className="text-3xl font-semibold mb-1 text-white">$12,458.32</div>
          <div className="text-green-500 text-sm">+2.4% today</div>
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-4 gap-3">
          {quickActions.map((action, i) => (
            <button 
              key={i} 
              className="flex flex-col items-center p-3 bg-[#2A2A2A] rounded-xl hover:bg-[#333333] transition-colors"
              onClick={() => navigate(action.path)}
            >
              <action.icon className="w-6 h-6 mb-1 text-white" />
              <span className="text-sm text-white">{action.label}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Assets */}
      <div className="flex-1 px-5 pt-6 overflow-y-auto">
        <h2 className="text-lg font-medium mb-4 text-white">Assets</h2>
        <div className="space-y-4">
          {/* Solana */}
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <div className="w-10 h-10 bg-[#2A2A2A] rounded-full flex items-center justify-center mr-3">
                <span className="text-lg text-white">◎</span>
              </div>
              <div>
                <div className="font-medium text-white">Solana</div>
                <div className="text-sm text-gray-400">SOL</div>
              </div>
            </div>
            <div className="text-right">
              <div className="font-medium text-white">32.4 SOL</div>
              <div className="text-sm text-gray-400">$2,431.23</div>
            </div>
          </div>

          {/* Ethereum */}
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <div className="w-10 h-10 bg-[#2A2A2A] rounded-full flex items-center justify-center mr-3">
                <span className="text-lg text-white">Ξ</span>
              </div>
              <div>
                <div className="font-medium text-white">Ethereum</div>
                <div className="text-sm text-gray-400">ETH</div>
              </div>
            </div>
            <div className="text-right">
              <div className="font-medium text-white">1.2 ETH</div>
              <div className="text-sm text-gray-400">$3,245.67</div>
            </div>
          </div>

          {/* USDC */}
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <div className="w-10 h-10 bg-[#2A2A2A] rounded-full flex items-center justify-center mr-3">
                <span className="text-lg text-white">$</span>
              </div>
              <div>
                <div className="font-medium text-white">USDC</div>
                <div className="text-sm text-gray-400">USDC</div>
              </div>
            </div>
            <div className="text-right">
              <div className="font-medium text-white">1,245 USDC</div>
              <div className="text-sm text-gray-400">$1,245.00</div>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom Navigation */}
      <div className="border-t border-[#2A2A2A]">
        <div className="flex justify-around py-3">
          {navItems.map((item, i) => (
            <button 
              key={i} 
              className={`flex flex-col items-center ${item.active ? "text-white" : "text-gray-400"} hover:text-white transition-colors`}
              onClick={() => navigate(item.path)}
            >
              <item.icon className="w-6 h-6 mb-1" />
              <span className="text-xs">{item.label}</span>
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}

