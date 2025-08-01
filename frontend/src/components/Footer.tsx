import React from 'react';
import { Shield, Twitter, Facebook, Linkedin, Mail, Sparkles } from 'lucide-react';

const Footer = () => {
  const footerLinks = {
    Product: ['Features', 'Pricing', 'Security', 'Downloads', 'Integrations'],
    Company: ['About Us', 'Careers', 'Press', 'Blog', 'Contact'],
    Support: ['Help Center', 'Community', 'API Docs', 'System Status', 'Security'],
    Legal: ['Privacy Policy', 'Terms of Service', 'Cookie Policy', 'GDPR', 'Compliance'],
  };

  return (
    <footer className="bg-background border-t border-border">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="grid md:grid-cols-5 gap-8">
          <div className="md:col-span-1">
            <div className="flex items-center space-x-2 mb-4">
              <div className="relative">
                <Shield className="h-8 w-8 text-primary" />
                <Sparkles className="h-3 w-3 text-primary absolute -top-1 -right-1 animate-pulse" />
              </div>
              <span className="text-xl font-bold bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">
                SecureVault
              </span>
            </div>
            <p className="text-muted-foreground mb-6">
              The most trusted password manager for individuals, families, and businesses.
            </p>
            <div className="flex space-x-4">
              <Twitter className="h-5 w-5 text-muted-foreground hover:text-primary cursor-pointer transition-all duration-300 hover:scale-110" />
              <Facebook className="h-5 w-5 text-muted-foreground hover:text-primary cursor-pointer transition-all duration-300 hover:scale-110" />
              <Linkedin className="h-5 w-5 text-muted-foreground hover:text-primary cursor-pointer transition-all duration-300 hover:scale-110" />
              <Mail className="h-5 w-5 text-muted-foreground hover:text-primary cursor-pointer transition-all duration-300 hover:scale-110" />
            </div>
          </div>

          {Object.entries(footerLinks).map(([category, links]) => (
            <div key={category}>
              <h3 className="font-semibold mb-4 text-foreground">{category}</h3>
              <ul className="space-y-2">
                {links.map((link) => (
                  <li key={link}>
                    <a
                      href="#"
                      className="text-muted-foreground hover:text-primary transition-colors text-sm hover:underline"
                    >
                      {link}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div className="border-t border-border mt-12 pt-8 flex flex-col md:flex-row justify-between items-center">
          <p className="text-muted-foreground text-sm">
            © 2025 SecureVault. All rights reserved.
          </p>
          <div className="flex items-center space-x-6 mt-4 md:mt-0">
            <span className="text-sm text-muted-foreground">🔒 Your data is encrypted</span>
            <span className="text-sm text-muted-foreground">✓ SOC 2 Certified</span>
            <span className="text-sm text-muted-foreground">🛡️ GDPR Compliant</span>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;