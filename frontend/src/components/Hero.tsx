import React from 'react';
import { ArrowRight, Download, Smartphone, Monitor, Tablet, Sparkles, Zap } from 'lucide-react';
import { Button } from '@/components/ui/button';

const Hero = () => {
  return (
    <section className="pt-16 pb-20 bg-gradient-to-br from-primary/5 via-background to-purple-50/30 relative overflow-hidden">
      {/* Animated background elements */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-40 -right-40 w-80 h-80 bg-primary/10 rounded-full blur-3xl animate-float"></div>
        <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-purple-500/10 rounded-full blur-3xl animate-float" style={{animationDelay: '1s'}}></div>
      </div>
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-4xl mx-auto relative z-10">
          <div className="inline-flex items-center space-x-2 bg-primary/10 text-primary px-4 py-2 rounded-full text-sm font-medium mb-6 animate-fade-in">
            <Sparkles className="h-4 w-4" />
            <span>New: Advanced AI-powered security monitoring</span>
          </div>
          
          <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold text-foreground mb-6 leading-tight animate-fade-in">
            Your passwords,
            <span className="bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent"> perfectly secure</span> and
            <span className="bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent"> always accessible</span>
          </h1>
          
          <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto leading-relaxed animate-fade-in" style={{animationDelay: '0.2s'}}>
            Stop reusing weak passwords. SecureVault generates, stores, and autofills 
            complex passwords across all your devices with military-grade encryption.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center mb-12 animate-fade-in" style={{animationDelay: '0.4s'}}>
            <Button size="lg" className="bg-gradient-to-r from-primary to-purple-600 hover:from-primary/90 hover:to-purple-600/90 transition-all duration-300 transform hover:scale-105 shadow-lg hover:shadow-xl font-semibold group">
              <span>Start Free Trial</span>
              <ArrowRight className="h-5 w-5 group-hover:translate-x-1 transition-transform" />
            </Button>
            <Button variant="outline" size="lg" className="border-2 hover:border-primary hover:text-primary transition-all duration-300 font-semibold group">
              <Download className="h-5 w-5" />
              <span>Download App</span>
            </Button>
          </div>

          <div className="flex justify-center items-center space-x-8 text-muted-foreground animate-fade-in" style={{animationDelay: '0.6s'}}>
            <div className="flex items-center space-x-2">
              <Monitor className="h-6 w-6" />
              <span className="text-sm font-medium">Desktop</span>
            </div>
            <div className="flex items-center space-x-2">
              <Smartphone className="h-6 w-6" />
              <span className="text-sm font-medium">Mobile</span>
            </div>
            <div className="flex items-center space-x-2">
              <Tablet className="h-6 w-6" />
              <span className="text-sm font-medium">Tablet</span>
            </div>
          </div>
        </div>

        <div className="mt-16 relative animate-fade-in" style={{animationDelay: '0.8s'}}>
          <div className="bg-card/80 backdrop-blur-xl rounded-2xl shadow-2xl p-8 max-w-4xl mx-auto border border-border/50 relative overflow-hidden">
            {/* Subtle gradient overlay */}
            <div className="absolute inset-0 bg-gradient-to-br from-primary/5 to-purple-500/5 rounded-2xl"></div>
            
            <div className="bg-muted/50 rounded-lg p-6 relative z-10">
              <div className="flex items-center space-x-3 mb-4">
                <div className="flex space-x-2">
                  <div className="w-3 h-3 bg-red-400 rounded-full"></div>
                  <div className="w-3 h-3 bg-yellow-400 rounded-full"></div>
                  <div className="w-3 h-3 bg-green-400 rounded-full"></div>
                </div>
                <div className="bg-background rounded px-3 py-1 text-sm text-muted-foreground flex-1">
                  https://secure-banking.com/login
                </div>
              </div>
              <div className="bg-background rounded-lg p-6 space-y-4 border border-border/50">
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground font-medium">Email:</span>
                  <span className="text-foreground">john.doe@email.com</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground font-medium">Password:</span>
                  <span className="text-primary font-mono">••••••••••••••••</span>
                </div>
                <Button className="w-full bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700 group">
                  <Zap className="h-4 w-4 mr-2 group-hover:animate-pulse" />
                  Auto-filled by SecureVault ✓
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;