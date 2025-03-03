"use client"

import { useState } from "react"
import { X, Send, Users, Clock, ChevronDown, CheckCircle2 } from "lucide-react"
import { useNavigate } from 'react-router-dom'
import '../styles/scrollbar.css'

export default function CreateProposal() {
  const navigate = useNavigate();
  const [description, setDescription] = useState("")

  // Mock data for demo
  const requiredSigners = 2;
  const totalSigners = 3;
  const approvers = [
    { address: "GkXn...k9B3", status: "approved" },
    { address: "Hj7K...m4P2", status: "pending" },
    { address: "Lp3M...x8N5", status: "pending" }
  ];

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]">
        <div className="flex items-center space-x-2">
          <Send className="w-5 h-5 text-white" />
          <h2 className="text-lg font-medium text-white">Create Proposal</h2>
        </div>
        <button 
          onClick={() => navigate('/dashboard')}
          className="text-white hover:bg-[#2A2A2A] p-2 rounded-lg transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide p-4 space-y-4">
        {/* Transaction Type */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Transaction Type</label>
          <button className="w-full flex items-center justify-between p-3 bg-[#2A2A2A] rounded-lg hover:bg-[#333333] transition-colors">
            <div className="flex items-center space-x-3">
              <Send className="w-5 h-5 text-white" />
              <span className="text-white">Send Tokens</span>
            </div>
            <ChevronDown className="w-5 h-5 text-white" />
          </button>
        </div>

        {/* Description */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Description</label>
          <textarea
            placeholder="Enter proposal description"
            className="w-full p-3 bg-[#2A2A2A] text-white rounded-lg outline-none hover:bg-[#333333] transition-colors min-h-[100px] resize-none"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
        </div>

        {/* Transaction Details */}
        <div className="space-y-2">
          <label className="block text-sm text-gray-400">Transaction Details</label>
          <div className="bg-[#2A2A2A] rounded-lg p-4 space-y-3">
            <div className="flex justify-between items-center">
              <span className="text-gray-400">Amount</span>
              <div className="flex items-center space-x-2">
                <div className="w-5 h-5 rounded-full bg-black flex items-center justify-center">
                  <span className="text-white text-sm">◎</span>
                </div>
                <span className="text-white">50.00 SOL</span>
              </div>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-400">To</span>
              <span className="text-white">Hj7K...m4P2</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-400">Network Fee</span>
              <span className="text-white">0.000005 SOL</span>
            </div>
          </div>
        </div>

        {/* Approval Status */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <label className="text-sm text-gray-400">Required Approvals</label>
            <span className="text-sm text-white">{requiredSigners} out of {totalSigners}</span>
          </div>
          <div className="bg-[#2A2A2A] rounded-lg divide-y divide-[#333333]">
            {approvers.map((approver, index) => (
              <div key={index} className="flex items-center justify-between p-4">
                <div className="flex items-center space-x-3">
                  <Users className="w-5 h-5 text-gray-400" />
                  <span className="text-white">{approver.address}</span>
                </div>
                <div className="flex items-center space-x-2">
                  {approver.status === "approved" ? (
                    <>
                      <CheckCircle2 className="w-5 h-5 text-green-500" />
                      <span className="text-green-500">Approved</span>
                    </>
                  ) : (
                    <>
                      <Clock className="w-5 h-5 text-yellow-500" />
                      <span className="text-yellow-500">Pending</span>
                    </>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[#2A2A2A]">
        <button 
          onClick={() => navigate('/confirm-transaction')}
          className="w-full bg-[#2A2A2A] text-white py-3 rounded-lg font-medium hover:bg-[#333333] transition-colors"
        >
          Create Proposal
        </button>
      </div>
    </div>
  )
}
