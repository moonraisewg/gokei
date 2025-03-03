"use client"

import { Wallet, Plus, Users, ChevronRight, Clock } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function MultisigList() {
  const navigate = useNavigate();

  // Mock data for demo
  const multisigWallets = [
    {
      name: "Team Wallet",
      address: "GkXn...k9B3",
      members: 3,
      requiredSigners: 2,
      pendingProposals: 2
    },
    {
      name: "DAO Treasury",
      address: "Hj7K...m4P2",
      members: 5,
      requiredSigners: 3,
      pendingProposals: 0
    },
    {
      name: "Project Fund",
      address: "Lp3M...x8N5",
      members: 4,
      requiredSigners: 2,
      pendingProposals: 1
    }
  ];

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Wallet className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Multisig Wallets</h2>
        </div>
        <button 
          onClick={() => navigate('/create-multisig')}
          className="flex items-center space-x-1 bg-[#2A2A2A] text-white px-3 py-2 rounded-lg hover:bg-[#333333] transition-colors"
        >
          <Plus className="w-4 h-4" />
          <span className="text-sm">New Wallet</span>
        </button>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-4">
        {multisigWallets.map((wallet, index) => (
          <button
            key={index}
            onClick={() => navigate('/create-proposal')}
            className="w-full bg-[#2A2A2A] rounded-lg p-4 hover:bg-[#333333] transition-colors"
          >
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-[#333333] rounded-full flex items-center justify-center">
                  <Wallet className="w-5 h-5 text-white" />
                </div>
                <div className="text-left">
                  <div className="text-white font-medium">{wallet.name}</div>
                  <div className="text-sm text-gray-400">{wallet.address}</div>
                </div>
              </div>
              <ChevronRight className="w-5 h-5 text-gray-400" />
            </div>
            
            <div className="flex items-center justify-between text-sm">
              <div className="flex items-center space-x-1 text-gray-400">
                <Users className="w-4 h-4" />
                <span>{wallet.requiredSigners} out of {wallet.members} signers</span>
              </div>
              {wallet.pendingProposals > 0 && (
                <div className="flex items-center space-x-1 text-yellow-500">
                  <Clock className="w-4 h-4" />
                  <span>{wallet.pendingProposals} pending</span>
                </div>
              )}
            </div>
          </button>
        ))}
      </div>
    </div>
  )
}
