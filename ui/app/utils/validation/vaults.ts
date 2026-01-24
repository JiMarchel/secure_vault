import z from "zod";

const title = z
  .string()
  .min(1, "Title is required")
  .max(64, "Title must be at most 64 characters long");

const usernameOrEmail = z
  .string()
  .max(64, "Username or email must be at most 64 characters long");
const password = z
  .string()
  .max(64, "Password must be at most 64 characters long");
const websiteOrApp = z
  .string()
  .max(64, "Website or app must be at most 64 characters long");

const id = z.uuid({ version: "v4" });
const note = z.string();

const cardHolderName = z
  .string()
  .max(64, "Card holder name must be at most 64 characters long");
const cardNumber = z
  .string()
  .regex(/^[\d\s]*$/, "Card number must contain only digits and spaces")
  .max(64, "Card number must be at most 64 characters long");
const cardExpirationDate = z
  .string()
  .max(64, "Card expiration date must be at most 64 characters long");
const cardCvv = z
  .string()
  .regex(/^\d*$/, "CVV must contain only digits")
  .max(4, "Card CVV must be at most 4 characters long");
const pin = z
  .string()
  .regex(/^\d*$/, "PIN must contain only digits")
  .max(12, "Card PIN must be at most 12 characters long");
const zipOrPostalCode = z
  .string()
  .max(64, "Zip or postal code must be at most 64 characters long");

const fullName = z
  .string()
  .max(64, "Full name must be at most 64 characters long");
const email = z.string().max(64, "Email must be at most 64 characters long");
const phoneNumber = z
  .string()
  .max(64, "Phone number must be at most 64 characters long");
const address = z
  .string()
  .max(64, "Address must be at most 64 characters long");
const city = z.string().max(64, "City must be at most 64 characters long");
const state = z.string().max(64, "State must be at most 64 characters long");
const country = z
  .string()
  .max(64, "Country must be at most 64 characters long");

export const addPassword = z.object({
  title,
  usernameOrEmail,
  password,
  websiteOrApp,
});

export const updatePassword = z.object({
  id,
  title,
  usernameOrEmail,
  password,
  websiteOrApp,
});

export const addNote = z.object({
  title,
  note,
});

export const updateNote = z.object({
  id,
  title,
  note,
});

export const addCreditCard = z.object({
  title,
  cardHolderName,
  cardNumber,
  cardExpirationDate,
  cardCvv,
  pin,
  zipOrPostalCode,
});

export const updateCreditCard = z.object({
  id,
  title,
  cardHolderName,
  cardNumber,
  cardExpirationDate,
  cardCvv,
  pin,
  zipOrPostalCode,
});

export const addContact = z.object({
  title,
  fullName,
  email,
  phoneNumber,
  address,
  city,
  state,
  country,
});

export const updateContact = z.object({
  id,
  title,
  fullName,
  email,
  phoneNumber,
  address,
  city,
  state,
  country,
});

export type addPasswordType = z.infer<typeof addPassword>;
export type updatePasswordType = z.infer<typeof updatePassword>;
export type addNoteType = z.infer<typeof addNote>;
export type updateNoteType = z.infer<typeof updateNote>;
export type addCreditCardType = z.infer<typeof addCreditCard>;
export type updateCreditCardType = z.infer<typeof updateCreditCard>;
export type addContactType = z.infer<typeof addContact>;
export type updateContactType = z.infer<typeof updateContact>;
