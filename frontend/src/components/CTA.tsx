import React from 'react';
import { ArrowRight, Download, Sparkles } from 'lucide-react';
import { Button } from '@/components/ui/button';

const CTA = () => {
  return (
    <section className="py-20 bg-gradient-to-r from-primary via-purple-600 to-pink-600 relative overflow-hidden">
      {/* Animated background elements */}
      <div className="absolute inset-0">
        <div className="absolute top-10 left-10 w-32 h-32 bg-white/10 rounded-full blur-2xl animate-float"></div>
        <div className="absolute bottom-10 right-10 w-40 h-40 bg-white/5 rounded-full blur-3xl animate-float" style={{animationDelay: '1s'}}></div>
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-white/5 rounded-full blur-3xl animate-float" style={{animationDelay: '2s'}}></div>
      </div>
      
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
        <div className="inline-flex items-center space-x-2 bg-white/20 backdrop-blur-sm text-white px-4 py-2 rounded-full text-sm font-medium mb-6 animate-fade-in">
          <Sparkles className="h-4 w-4" />
          <span>Limited time: 50% off first year</span>
        </div>
        
        <h2 className="text-3xl sm:text-4xl font-bold text-white mb-6 animate-fade-in" style={{animationDelay: '0.2s'}}>
          Ready to secure your digital life?
        </h2>
        <p className="text-xl text-white/80 mb-8 max-w-2xl mx-auto animate-fade-in" style={{animationDelay: '0.4s'}}>
          Join over 100,000 users who trust SecureVault to protect their passwords. 
          Start your free trial today - no credit card required.
        </p>
        
        <div className="flex flex-col sm:flex-row gap-4 justify-center animate-fade-in" style={{animationDelay: '0.6s'}}>
          <Button size="lg" className="bg-white text-primary hover:bg-white/90 transition-all duration-300 transform hover:scale-105 shadow-lg font-semibold group">
            <span>Start Free Trial</span>
            <ArrowRight className="h-5 w-5 group-hover:translate-x-1 transition-transform" />
          </Button>
          <Button variant="outline" size="lg" className="border-2 border-white text-white hover:bg-white hover:text-primary transition-all duration-300 font-semibold bg-transparent">
            <Download className="h-5 w-5" />
            <span>Download App</span>
          </Button>
        </div>

        <p className="text-white/70 text-sm mt-6 animate-fade-in" style={{animationDelay: '0.8s'}}>
          30-day money-back guarantee • Cancel anytime • No setup fees
        </p>
      </div>
    </section>
  );
};

export default CTA;