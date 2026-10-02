import sys
sys.path.append('services/ai_worker/TripoSR')
try:
    from tsr.system import TSR
    print("SUCCESS: TripoSR imported successfully")
except ImportError as e:
    print(f"ERROR: {e}")
    import traceback
    traceback.print_exc()