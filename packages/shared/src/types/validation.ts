/** Describes a single validation failure. */
export interface ValidationError {
  /** The name of the property that failed validation */
  field: string;
  /** Human-readable description of the failure */
  message: string;
}

/** Aggregated result of running validations. */
export interface ValidationResult {
  /** Whether all validations passed */
  valid: boolean;
  /** List of individual validation errors (empty when valid) */
  errors: ValidationError[];
}
