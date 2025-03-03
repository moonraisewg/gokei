"use client"

import { X, Check, Clock } from "lucide-react"

interface Approver {
  name: string
  status: "approved" | "pending"
  timestamp?: string
  avatar: string
}

export default function MultiSignatureApproval() {
  const approvers: Approver[] = [
    {
      name: "Alex Thompson",
      status: "approved",
      timestamp: "Jan 15, 2025 14:30",
      avatar: "👨",
    },
    {
      name: "Sarah Chen",
      status: "pending",
      avatar: "👩",
    },
    {
      name: "Marcus Rodriguez",
      status: "pending",
      avatar: "👨",
    },
  ]

  return (
    <div className="w-full max-w-md bg-black rounded-xl shadow-lg">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b">
        <h1 className="text-lg font-medium">Multi-Signature Approval</h1>
        <button className="text-white hover:text-white">
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Transaction Details */}
      <div className="p-4 space-y-4">
        <div className="flex justify-between">
          <span className="text-white">Amount</span>
          <span className="font-medium">2.5 ETH</span>
        </div>
        <div className="flex justify-between">
          <span className="text-white">Recipient</span>
          <span className="font-medium">0x1234...5678</span>
        </div>
        <div className="space-y-1">
          <span className="text-white">Reason</span>
          <p className="text-sm">Treasury management - Q1 2025 operational expenses</p>
        </div>
      </div>

      {/* Approvals */}
      <div className="px-4 pt-4">
        <div className="flex justify-between items-center mb-4">
          <span className="font-medium">Approvals Required</span>
          <span className="font-medium">2 of 3</span>
        </div>

        <div className="space-y-4">
          {approvers.map((approver, index) => (
            <div key={index} className="flex items-center justify-between">
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 rounded-full bg-black flex items-center justify-center">
                  <span className="text-lg">{approver.avatar}</span>
                </div>
                <div>
                  <div className="font-medium">{approver.name}</div>
                  <div className="text-sm text-white flex items-center space-x-1">
                    <span>{approver.status === "approved" ? "Approved • " : "Pending"}</span>
                    {approver.timestamp && <span>{approver.timestamp}</span>}
                  </div>
                </div>
              </div>
              {approver.status === "approved" ? (
                <Check className="w-5 h-5 text-green-500" />
              ) : (
                <Clock className="w-5 h-5 text-white" />
              )}
            </div>
          ))}
        </div>
      </div>

      {/* Action Button */}
      <div className="p-4">
        <button className="w-full bg-black text-white rounded-lg py-3 font-medium hover:bg-black/90">
          Sign Transaction
        </button>
      </div>
    </div>
  )
}

