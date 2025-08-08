export interface SignUpFormData {
  username?: string;
  email?: string;
}

export type MessageApi = {
  messageApi?: string;
};

export interface signUpActionResponse {
  errors?: {
    [K in keyof SignUpFormData]?: string[];
  };
  inputs?: SignUpFormData;
  messageApi?: string;
}

export interface OtpVerifFormData {
  otp_code?: string;
}

export interface OtpVerifActionResponse {
  errors?: {
    [K in keyof OtpVerifFormData]?: string[];
  };
  inputs?: OtpVerifFormData;
  messageApi?: string;
}

export interface VerifPasswordFormData {
  password?: string;
  confirm_password?: string;
}

export interface VerifPasswordActionResponse {
  errors?: {
    [K in keyof VerifPasswordFormData]?: string[];
  };
  inputs?: VerifPasswordFormData;
  messageApi?: string;
}
