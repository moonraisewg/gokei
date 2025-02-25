
import Header from './components/layout/Header';
import Home from './pages/Home';

const App = () => {
  return (
    <div className="min-h-screen bg-gray-900">
      <Header />
      <main className="container mx-auto px-4 py-8">
        <Home />
      </main>
    </div>
  );
};

export default App;