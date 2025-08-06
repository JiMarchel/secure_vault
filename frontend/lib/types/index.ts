export interface SignUpFormData {
  username?: string;
  email?: string;
}

export interface signUpActionResponse {
  errors?: {
    [K in keyof SignUpFormData]?: string[];
  };
  inputs?: SignUpFormData;
  messageApi?: string;
}

export interface OtpVerifFormData {
  otp?: string;
}

export interface OtpVerifActionResponse {
  errors?: {
    [K in keyof OtpVerifFormData]?: string[];
  };
  inputs?: OtpVerifFormData;
  messageApi?: string;
}
