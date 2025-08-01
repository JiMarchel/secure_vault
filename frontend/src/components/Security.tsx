import React from 'react';
import { Shield, Lock, Server, FileCheck } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

const Security = () => {
  const securityFeatures = [
    {
      icon: Shield,
      title: 'Zero-Knowledge Architecture',
      description: 'We can never see your passwords. Your master password is the only key to decrypt your data.',
    },
    {
      icon: Lock,
      title: 'AES-256 Encryption',
      description: 'Bank-level encryption protects your data both in transit and at rest with industry-standard protocols.',
    },
    {
      icon: Server,
      title: 'Secure Cloud Infrastructure',
      description: 'Your encrypted data is stored on secure servers with 99.9% uptime and regular security audits.',
    },
    {
      icon: FileCheck,
      title: 'Regular Security Audits',
      description: 'Independent security firms regularly audit our systems to ensure the highest level of protection.',
    },
  ];

  const certifications = [
    'SOC 2 Type II Certified',
    'ISO 27001 Compliant',
    'GDPR Compliant',
    'Privacy Shield Certified',
  ];

  return (
    <section id="security" className="py-20 bg-muted/30 relative overflow-hidden">
      {/* Animated background elements */}
      <div className="absolute top-20 right-20 w-64 h-64 bg-primary/5 rounded-full blur-3xl animate-float"></div>
      <div className="absolute bottom-20 left-20 w-64 h-64 bg-purple-500/5 rounded-full blur-3xl animate-float" style={{animationDelay: '2s'}}></div>
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16 animate-fade-in">
          <h2 className="text-3xl sm:text-4xl font-bold text-foreground mb-4">
            Security you can trust
          </h2>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
            Your security is our top priority. We use industry-leading practices to protect your sensitive information.
          </p>
        </div>

        <div className="grid md:grid-cols-2 gap-8 mb-16 relative z-10">
          {securityFeatures.map((feature, index) => (
            <div key={index} className="flex items-start space-x-4 animate-fade-in" style={{animationDelay: `${index * 0.2}s`}}>
              <div className="bg-primary/10 w-12 h-12 rounded-lg flex items-center justify-center flex-shrink-0 hover:bg-primary/20 transition-colors duration-300">
                <feature.icon className="h-6 w-6 text-primary" />
              </div>
              <div>
                <h3 className="text-xl font-semibold text-foreground mb-2">
                  {feature.title}
                </h3>
                <p className="text-muted-foreground leading-relaxed">
                  {feature.description}
                </p>
              </div>
            </div>
          ))}
        </div>

        <Card className="p-8 text-center bg-card/80 backdrop-blur-sm border-border/50 animate-fade-in" style={{animationDelay: '0.8s'}}>
          <h3 className="text-2xl font-bold text-foreground mb-6">
            Trusted by security professionals worldwide
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {certifications.map((cert, index) => (
              <Card
                key={index}
                className="p-4 hover:shadow-md transition-all duration-300 hover:scale-105 bg-background/80"
              >
                <span className="text-sm font-medium text-foreground">{cert}</span>
              </Card>
            ))}
          </div>
          <p className="text-muted-foreground mt-6">
            Independently verified by leading security organizations
          </p>
        </Card>
      </div>
    </section>
  );
};

export default Security;