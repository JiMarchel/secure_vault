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
