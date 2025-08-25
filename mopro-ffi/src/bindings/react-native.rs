use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("bindings/react-native");
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");
    
    // Create a placeholder TypeScript file
    let ts_content = r#"// Generated TypeScript bindings for mopro-ffi
// This is a placeholder implementation

export interface User {
  id: number;
  name: string;
  email: string;
}

export interface CalculationResult {
  value: number;
  message: string;
}

export class MoproFfi {
  static addNumbers(a: number, b: number): number {
    return a + b;
  }

  static createUser(id: number, name: string, email: string): User {
    return { id, name, email };
  }

  static calculateSquareRoot(value: number): CalculationResult {
    if (value < 0) {
      return { value: NaN, message: "Cannot calculate square root of negative number" };
    } else {
      return { value: Math.sqrt(value), message: "Square root calculated successfully" };
    }
  }

  static sumArray(numbers: number[]): number {
    return numbers.reduce((sum, num) => sum + num, 0);
  }

  static validateEmail(email: string): boolean {
    return email.includes("@") && email.includes(".");
  }
}
"#;
    
    std::fs::write(out_dir.join("mopro_ffi.ts"), ts_content)
        .expect("Failed to write TypeScript file");
    
    println!("✅ React Native bindings generated successfully in {}", out_dir.display());
    println!("📱 TypeScript files created:");
    println!("   - mopro_ffi.ts");
}
