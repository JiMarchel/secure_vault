import React, { useState } from 'react';
import { Menu, X, Shield, Sparkles } from 'lucide-react';
import { Button } from '@/components/ui/button';

const Header = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <header className="fixed top-0 w-full bg-background/80 backdrop-blur-xl border-b border-border z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <div className="flex items-center space-x-2">
            <div className="relative">
              <Shield className="h-8 w-8 text-primary" />
              <Sparkles className="h-3 w-3 text-primary absolute -top-1 -right-1 animate-pulse" />
            </div>
            <span className="text-xl font-bold bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">
              SecureVault
            </span>
          </div>
          
          <nav className="hidden md:flex items-center space-x-8">
            <a href="#features" className="text-muted-foreground hover:text-primary transition-all duration-300 hover:scale-105">Features</a>
            <a href="#security" className="text-muted-foreground hover:text-primary transition-all duration-300 hover:scale-105">Security</a>
            <a href="#support" className="text-muted-foreground hover:text-primary transition-all duration-300 hover:scale-105">Support</a>
          </nav>

          <div className="hidden md:flex items-center space-x-4">
            <Button variant="ghost" className="font-medium">
              Sign In
            </Button>
            <Button className="bg-gradient-to-r from-primary to-purple-600 hover:from-primary/90 hover:to-purple-600/90 transition-all duration-300 transform hover:scale-105 shadow-lg">
              Get Started
            </Button>
          </div>

          <Button
            variant="ghost"
            size="icon"
            className="md:hidden"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            {isMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
          </Button>
        </div>

        {isMenuOpen && (
          <div className="md:hidden animate-slide-in">
            <div className="px-2 pt-2 pb-3 space-y-1 bg-background/95 backdrop-blur-xl border-t border-border">
              <a href="#features" className="block px-3 py-2 text-muted-foreground hover:text-primary transition-colors">Features</a>
              <a href="#security" className="block px-3 py-2 text-muted-foreground hover:text-primary transition-colors">Security</a>
              <a href="#support" className="block px-3 py-2 text-muted-foreground hover:text-primary transition-colors">Support</a>
              <div className="pt-4 border-t border-border">
                <Button variant="ghost" className="w-full justify-start">
                  Sign In
                </Button>
                <Button className="w-full mt-2 bg-gradient-to-r from-primary to-purple-600">
                  Get Started
                </Button>
              </div>
            </div>
          </div>
        )}
      </div>
    </header>
  );
};

export default Header;