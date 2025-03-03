"use client"

import { useState } from "react"
import { X, Trash2, ChevronDown, Wallet, Plus } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function CreateMultisigWallet() {
  const navigate = useNavigate();
  const [walletName, setWalletName] = useState("")
  const [members, setMembers] = useState(["", ""])

  const addMember = () => {
    setMembers([...members, ""])
  }

  const removeMember = (index: number) => {
    setMembers(members.filter((_, i) => i !== index))
  }

  const updateMember = (index: number, value: string) => {
    const newMembers = [...members]
    newMembers[index] = value
    setMembers(newMembers)
  }

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Wallet className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Create Multisig Wallet</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Form */}
      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-4">
        {/* Wallet Name */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Wallet Name</label>
          <input
            type="text"
            placeholder="Enter wallet name"
            className="w-full p-3 rounded-lg bg-[#2A2A2A] text-white outline-none hover:bg-[#333333] transition-colors"
            value={walletName}
            onChange={(e) => setWalletName(e.target.value)}
          />
        </div>

        {/* Wallet Members */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Wallet Members</label>
          <div className="space-y-2">
            {members.map((member, index) => (
              <div key={index} className="flex space-x-2">
                <input
                  type="text"
                  placeholder="Enter wallet address"
                  className="flex-1 p-3 rounded-lg bg-[#2A2A2A] text-white outline-none hover:bg-[#333333] transition-colors"
                  value={member}
                  onChange={(e) => updateMember(index, e.target.value)}
                />
                <button 
                  onClick={() => removeMember(index)} 
                  className="p-3 text-gray-400 hover:text-white bg-[#2A2A2A] hover:bg-[#333333] rounded-lg transition-colors"
                >
                  <Trash2 className="w-5 h-5" />
                </button>
              </div>
            ))}
          </div>
          <button 
            onClick={addMember} 
            className="flex items-center space-x-2 text-sm text-gray-400 hover:text-white transition-colors p-2"
          >
            <Plus className="w-4 h-4" />
            <span>Add Member</span>
          </button>
        </div>

        {/* Required Signatures */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Required Signatures</label>
          <button className="w-full p-3 rounded-lg bg-[#2A2A2A] text-white hover:bg-[#333333] transition-colors flex items-center justify-between">
            <span>1 out of {members.length}</span>
            <ChevronDown className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button 
          onClick={() => navigate('/dashboard')}
          className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors"
        >
          Create Wallet
        </button>
      </div>
    </div>
  )
}

