import React from 'react';
import { Shield, Zap, FolderSync as Sync, Key, Eye, Globe } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

const Features = () => {
  const features = [
    {
      icon: Shield,
      title: 'Military-Grade Encryption',
      description: 'AES-256 encryption ensures your data is protected with the same security standards used by governments and banks.',
    },
    {
      icon: Zap,
      title: 'Instant Auto-Fill',
      description: 'Save time with lightning-fast auto-fill across all websites and apps. No more typing complex passwords.',
    },
    {
      icon: Sync,
      title: 'Cross-Device Sync',
      description: 'Access your passwords seamlessly across all devices - desktop, mobile, and tablet with real-time synchronization.',
    },
    {
      icon: Key,
      title: 'Strong Password Generator',
      description: 'Generate unique, complex passwords for every account with customizable length and character requirements.',
    },
    {
      icon: Eye,
      title: 'Security Monitoring',
      description: 'Get alerts for weak, reused, or compromised passwords with our advanced security dashboard.',
    },
    {
      icon: Globe,
      title: 'Secure Sharing',
      description: 'Safely share passwords and sensitive information with family or team members using encrypted channels.',
    },
  ];

  return (
    <section id="features" className="py-20 bg-background relative">
      {/* Background pattern */}
      <div className="absolute inset-0 bg-grid-pattern opacity-5"></div>
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16 animate-fade-in">
          <h2 className="text-3xl sm:text-4xl font-bold text-foreground mb-4">
            Everything you need for password security
          </h2>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
            Comprehensive features designed to keep your digital life secure and convenient.
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8 relative z-10">
          {features.map((feature, index) => (
            <Card
              key={index}
              className="group hover:shadow-xl transition-all duration-500 hover:scale-105 border-border/50 bg-card/50 backdrop-blur-sm animate-fade-in"
              style={{animationDelay: `${index * 0.1}s`}}
            >
              <CardHeader>
                <div className="bg-primary/10 w-12 h-12 rounded-lg flex items-center justify-center mb-4 group-hover:bg-primary/20 transition-all duration-300 group-hover:scale-110">
                  <feature.icon className="h-6 w-6 text-primary" />
                </div>
                <CardTitle className="text-xl group-hover:text-primary transition-colors duration-300">
                  {feature.title}
                </CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-base leading-relaxed">
                  {feature.description}
                </CardDescription>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
};

export default Features;