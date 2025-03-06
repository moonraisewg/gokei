"use client"

import { ArrowLeft, Shield, Bell, Moon, HelpCircle, LogOut } from "lucide-react"
import { useNavigate } from 'react-router-dom'

export default function Settings() {
  const navigate = useNavigate();

  const settingItems = [
    { icon: Shield, label: "Security", description: "Protect your wallet" },
    { icon: Bell, label: "Notifications", description: "Manage alerts" },
    { icon: Moon, label: "Appearance", description: "Dark mode & theme" },
    { icon: HelpCircle, label: "Help & Support", description: "Get assistance" },
  ];

  return (
    <div className="h-full flex flex-col bg-black">
      {/* Header */}
      <div className="p-4 flex items-center space-x-2 border-b border-[#2A2A2A]">
        <button 
          onClick={() => navigate('/dashboard')}
          className="hover:bg-[#2A2A2A] p-2 rounded-lg"
        >
          <ArrowLeft className="h-5 w-5 text-white" />
        </button>
        <h1 className="text-white text-lg">Settings</h1>
      </div>

      {/* Settings List */}
      <div className="flex-1 overflow-y-auto">
        <div className="p-4 space-y-2">
          {settingItems.map((item, i) => (
            <button
              key={i}
              className="w-full flex items-center p-4 bg-[#2A2A2A] rounded-lg hover:bg-[#333333] transition-colors"
            >
              <item.icon className="w-5 h-5 text-white mr-3" />
              <div className="text-left">
                <div className="text-white">{item.label}</div>
                <div className="text-sm text-gray-400">{item.description}</div>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Logout Button */}
      <div className="p-4 border-t border-[#2A2A2A]">
        <button
          onClick={() => navigate('/onboarding')}
          className="w-full flex items-center justify-center p-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
        >
          <LogOut className="w-5 h-5 mr-2" />
          <span>Log Out</span>
        </button>
      </div>
    </div>
  )
}
