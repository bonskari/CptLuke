import requests
import os
import sys
from PIL import Image
from io import BytesIO

def generate_image(prompt: str, output_path: str, api_token: str):
    """
    Generates an image using the Hugging Face Inference API for Stable Diffusion,
    then converts it to PNG.

    Args:
        prompt (str): The text prompt for image generation.
        output_path (str): The full path to save the generated PNG image.
    """
    api_token = os.getenv("HF_API_TOKEN")
    if not api_token:
        if len(sys.argv) > 1:
            api_token = sys.argv[1]
        else:
            print("Error: HF_API_TOKEN environment variable not set and no token provided as argument.", file=sys.stderr)
            print("Please get your Hugging Face API token from https://huggingface.co/settings/tokens", file=sys.stderr)
            sys.exit(1)

    API_URL = "https://api-inference.huggingface.co/models/stabilityai/stable-diffusion-xl-base-1.0"
    headers = {"Authorization": f"Bearer {api_token}"}

    payload = {"inputs": prompt}

    try:
        response = requests.post(API_URL, headers=headers, json=payload)
        response.raise_for_status() # Raise an exception for HTTP errors

        content_type = response.headers.get("Content-Type")
        print(f"Response Content-Type: {content_type}")
        print(f"First 20 bytes of response content: {response.content[:20]}")

        if "image" not in content_type:
            print(f"Error: Expected image, but received {content_type}. Response: {response.text}", file=sys.stderr)
            sys.exit(1)

        image_data = response.content
        img = Image.open(BytesIO(image_data))
        img.save(output_path, "PNG")
        print(f"Converted image saved to {output_path}")

    except requests.exceptions.RequestException as e:
        print(f"Error during API request: {e}", file=sys.stderr)
        if response.status_code == 401:
            print("Check your HF_API_TOKEN. It might be invalid or expired.", file=sys.stderr)
        elif response.status_code == 503:
            print("Model is currently loading. Please try again in a few moments.", file=sys.stderr)
        print(f"Response content: {response.text}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python generate_texture.py <HF_API_TOKEN> <PROMPT> <OUTPUT_FILENAME>", file=sys.stderr)
        sys.exit(1)

    api_token = sys.argv[1]
    prompt = sys.argv[2]
    filename = sys.argv[3]

    base_dir = os.path.join("assets", "textures")
    os.makedirs(base_dir, exist_ok=True)

    full_path = os.path.join(base_dir, filename)
    print(f"Generating {filename}...")
    generate_image(prompt, full_path, api_token)