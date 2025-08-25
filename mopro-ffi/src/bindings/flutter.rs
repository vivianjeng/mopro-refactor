use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("bindings/flutter");
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");
    
    // Create a placeholder Dart file
    let dart_content = r#"// Generated Dart bindings for mopro-ffi
// This is a placeholder implementation

class User {
  final int id;
  final String name;
  final String email;

  User({required this.id, required this.name, required this.email});
}

class CalculationResult {
  final double value;
  final String message;

  CalculationResult({required this.value, required this.message});
}

class MoproFfi {
  static double addNumbers(double a, double b) {
    return a + b;
  }

  static User createUser(int id, String name, String email) {
    return User(id: id, name: name, email: email);
  }

  static CalculationResult calculateSquareRoot(double value) {
    if (value < 0) {
      return CalculationResult(
        value: double.nan,
        message: "Cannot calculate square root of negative number",
      );
    } else {
      return CalculationResult(
        value: value.sqrt(),
        message: "Square root calculated successfully",
      );
    }
  }

  static double sumArray(List<double> numbers) {
    return numbers.fold(0.0, (sum, num) => sum + num);
  }

  static bool validateEmail(String email) {
    return email.contains("@") && email.contains(".");
  }
}
"#;
    
    std::fs::write(out_dir.join("mopro_ffi.dart"), dart_content)
        .expect("Failed to write Dart file");
    
    println!("✅ Flutter bindings generated successfully in {}", out_dir.display());
    println!("📱 Dart files created:");
    println!("   - mopro_ffi.dart");
}
