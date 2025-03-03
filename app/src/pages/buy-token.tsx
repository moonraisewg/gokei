"use client"

import { useState } from "react"
import { X, CreditCard, ChevronDown } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function BuyToken() {
  const navigate = useNavigate();
  const [amount, setAmount] = useState("");

  const paymentMethods = [
    { name: "Credit Card", icon: "💳", description: "Visa, Mastercard" },
    { name: "Apple Pay", icon: "", description: "Pay with Apple Pay" },
    { name: "Google Pay", icon: "", description: "Pay with Google Pay" },
  ];

  const quickAmounts = ["$100", "$200", "$500", "$1000"];

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <CreditCard className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Buy Tokens</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-4">
        {/* Select Token */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Select Token</label>
          <button className="w-full flex items-center justify-between p-4 bg-[#2A2A2A] rounded-lg hover:bg-[#333333] transition-colors">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 rounded-full bg-black flex items-center justify-center">
                <span className="text-purple-500">◎</span>
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
          <div className="flex justify-between items-center">
            <label className="text-sm text-gray-400">Amount</label>
            <div className="flex space-x-2">
              {quickAmounts.map((value) => (
                <button 
                  key={value}
                  onClick={() => setAmount(value.replace("$", ""))}
                  className="px-3 py-1 text-sm bg-[#2A2A2A] text-white rounded-lg hover:bg-[#333333] transition-colors"
                >
                  {value}
                </button>
              ))}
            </div>
          </div>
          <div className="p-4 bg-[#2A2A2A] rounded-lg space-y-2 hover:bg-[#333333] transition-colors">
            <div className="relative">
              <input
                type="text"
                placeholder="Enter amount"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="w-full bg-transparent text-white text-2xl outline-none"
              />
              <div className="absolute right-0 top-1/2 -translate-y-1/2">
                <span className="text-sm text-gray-400">USD</span>
              </div>
            </div>
            <div className="text-sm text-gray-400">You will receive: ≈ 5.23 SOL</div>
          </div>
        </div>

        {/* Payment Method */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Payment Method</label>
          <div className="space-y-2">
            {paymentMethods.map((method, index) => (
              <button
                key={index}
                className="w-full flex items-center justify-between p-4 bg-[#2A2A2A] rounded-lg hover:bg-[#333333] transition-colors"
              >
                <div className="flex items-center space-x-3">
                  <div className="w-10 h-10 bg-[#333333] rounded-lg flex items-center justify-center">
                    <span className="text-xl">{method.icon}</span>
                  </div>
                  <div className="text-left">
                    <div className="text-white">{method.name}</div>
                    <div className="text-sm text-gray-400">{method.description}</div>
                  </div>
                </div>
                <div className="w-5 h-5 rounded-full border border-white flex items-center justify-center">
                  {index === 0 && <div className="w-3 h-3 rounded-full bg-white"></div>}
                </div>
              </button>
            ))}
          </div>
        </div>

        {/* Network Fee */}
        <div className="p-4 bg-[#2A2A2A] rounded-lg space-y-2 text-sm hover:bg-[#333333] transition-colors">
          <div className="flex justify-between">
            <span className="text-gray-400">Network Fee</span>
            <span className="text-white">$2.50</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">Total Amount</span>
            <span className="text-white">${amount || "0.00"}</span>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors">
          Continue to Payment
        </button>
      </div>
    </div>
  )
}
