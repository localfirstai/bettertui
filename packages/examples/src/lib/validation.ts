/** A non-empty name is at least two characters. */
export function validateName(value: string): boolean {
  return value.length >= 2;
}

/** A basic RFC-5322-style email shape check. */
export function validateEmail(value: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(value);
}

/** A password is at least six characters. */
export function validatePassword(value: string): boolean {
  return value.length >= 6;
}
