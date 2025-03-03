import React from 'react';

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary';
  className?: string;
  disabled?: boolean;
}

const Button = ({ children, onClick, variant = 'primary', className = '', disabled = false }: ButtonProps) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`px-4 py-2 rounded-lg font-medium 
      ${variant === 'primary' ? 'bg-primary text-white' : 'bg-secondary text-black-200'}
      ${className}`}
    >
      {children}
    </button>
  );
};

export default Button;