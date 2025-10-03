#!/usr/bin/env python3
"""
Simple test for Pandora SDK Python bindings
"""

def main():
    print("🤖 Testing Pandora SDK Python Bindings")
    print("=" * 50)
    
    try:
        # Test basic import
        print("1. Testing basic import...")
        import pandora_sdk
        print("   ✓ pandora_sdk imported successfully")
        
        # Test functions
        print("\n2. Testing functions...")
        print(f"   Version: {pandora_sdk.get_version()}")
        print(f"   Greeting: {pandora_sdk.hello('Python Developer')}")
        print(f"   Simulation: {pandora_sdk.run_gridworld_simulation()}")
        
        print("\n🎉 All tests passed! Python bindings are working!")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        print("Python bindings need to be fixed.")
        return False
    
    return True

if __name__ == "__main__":
    main()
