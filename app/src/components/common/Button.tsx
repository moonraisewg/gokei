import React from 'react';

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary';
  className?: string;
}

const Button = ({ children, onClick, variant = 'primary', className = '' }: ButtonProps) => {
  return (
    <button
      onClick={onClick}
      className={`px-4 py-2 rounded-lg font-medium 
      ${variant === 'primary' ? 'bg-primary text-white' : 'bg-secondary text-gray-200'}
      ${className}`}
    >
      {children}
    </button>
  );
};

export default Button;