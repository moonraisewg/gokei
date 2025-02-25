
import WalletConnect from '../wallet/WalletConnect';

const Header = () => {
  return (
    <header className="bg-secondary p-4">
      <div className="container mx-auto flex justify-between items-center">
        <h1 className="text-xl font-bold">Moon Wallet</h1>
        <WalletConnect />
      </div>
    </header>
  );
};

export default Header;