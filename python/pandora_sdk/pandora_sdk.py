"""
Pandora SDK - Core functions from Rust uniffi bindings
"""

# Simple Python bindings for demonstration
def hello(name: str) -> str:
    """Say hello to a user"""
    return f"Hello from Pandora SDK, {name}!"

def get_version() -> str:
    """Get the SDK version"""
    return "Pandora Genesis SDK v1.0.0"

def run_gridworld_simulation() -> str:
    """Run a GridWorld simulation"""
    return "GridWorld simulation completed successfully!"

# Export all functions
__all__ = ['hello', 'get_version', 'run_gridworld_simulation']