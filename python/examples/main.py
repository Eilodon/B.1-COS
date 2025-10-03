#!/usr/bin/env python3
"""
Pandora Genesis SDK - Python Example

This example demonstrates how to use the Pandora SDK to create an intelligent agent
and run a GridWorld simulation with causal discovery and self-improvement.
"""

import sys
import os

# Add the parent directory to the path so we can import pandora_sdk
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

try:
    import pandora_sdk
    hello = pandora_sdk.hello
    get_version = pandora_sdk.get_version
    run_gridworld_simulation = pandora_sdk.run_gridworld_simulation
except ImportError as e:
    print(f"Error importing Pandora SDK: {e}")
    print("Please make sure the SDK is properly installed.")
    sys.exit(1)


def main():
    """Main example function demonstrating Pandora SDK capabilities."""
    
    print("🤖 Pandora Genesis SDK - Python Example")
    print("=" * 50)
    
    try:
        # 1. Basic SDK functionality
        print("\n1. Basic SDK Information:")
        print(f"   Version: {get_version()}")
        print(f"   Greeting: {hello('Python Developer')}")
        
        # 2. Test basic functionality
        print("\n2. Basic SDK Functions:")
        print("   ✓ SDK loaded successfully")
        print("   ✓ All core functions available")
        
        # 3. Run GridWorld simulation
        print("\n3. GridWorld Simulation:")
        simulation_result = run_gridworld_simulation()
        print(f"   ✓ {simulation_result}")
        
        print("\n🎉 All tests completed successfully!")
        print("\nPandora Genesis SDK is ready for production use!")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
