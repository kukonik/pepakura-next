import { ImageTo3DClient } from './imageTo3DClient'
import { GenerationResult, GenerationStatus } from './types'

// Мокаем fetch API
global.fetch = jest.fn()

describe('ImageTo3DClient', () => {
  let client: ImageTo3DClient
  const mockBaseUrl = 'http://localhost:3000/api'

  beforeEach(() => {
    client = new ImageTo3DClient(mockBaseUrl)
    ;(fetch as jest.Mock).mockClear()
  })

  describe('generateModel', () => {
    it('should generate model from base64 image and return generation result', async () => {
      const mockResult: GenerationResult = {
        modelId: 'test-model-id',
        status: 'processing',
        progress: 0,
        previews: []
      }

      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResult
      })

      const base64Image = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=='
      const result = await client.generateModel(base64Image)

      expect(result).toEqual(mockResult)
      expect(fetch).toHaveBeenCalledWith(
        `${mockBaseUrl}/image-to-3d/generate`,
        expect.objectContaining({
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ image: base64Image })
        })
      )
    })

    it('should generate model from file and return generation result', async () => {
      const mockResult: GenerationResult = {
        modelId: 'test-model-id',
        status: 'processing',
        progress: 0,
        previews: []
      }

      // Мокаем FileReader
      const mockFileReader = {
        readAsDataURL: jest.fn().mockImplementation(function(this: FileReader) {
          setTimeout(() => {
            // @ts-ignore
            this.onload?.({ target: { result: 'data:image/png;base64,test' } })
          }, 0)
        })
      }
      ;(global as any).FileReader = jest.fn(() => mockFileReader)

      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResult
      })

      const file = new File(['test'], 'test.png', { type: 'image/png' })
      const result = await client.generateModel(file)

      expect(result).toEqual(mockResult)
      expect(fetch).toHaveBeenCalledWith(
        `${mockBaseUrl}/image-to-3d/generate`,
        expect.objectContaining({
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ image: 'data:image/png;base64,test' })
        })
      )
    })

    it('should throw error when generation fails', async () => {
      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 500,
        statusText: 'Internal Server Error'
      })

      const base64Image = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=='
      await expect(client.generateModel(base64Image)).rejects.toThrow(
        'Failed to generate model: 500 Internal Server Error'
      )
    })
  })

  describe('getGenerationStatus', () => {
    it('should get generation status', async () => {
      const mockStatus: GenerationStatus = {
        status: 'processing',
        progress: 50
      }

      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockStatus
      })

      const result = await client.getGenerationStatus('test-model-id')

      expect(result).toEqual(mockStatus)
      expect(fetch).toHaveBeenCalledWith(
        `${mockBaseUrl}/image-to-3d/status/test-model-id`
      )
    })

    it('should throw error when getting status fails', async () => {
      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: 'Not Found'
      })

      await expect(
        client.getGenerationStatus('non-existent-model-id')
      ).rejects.toThrow('Failed to get generation status: 404 Not Found')
    })
  })

  describe('getGenerationResult', () => {
    it('should get generation result', async () => {
      const mockResult: GenerationResult = {
        modelId: 'test-model-id',
        status: 'completed',
        progress: 100,
        model: {
          vertices: [],
          faces: [],
          textures: []
        },
        previews: [
          {
            url: 'http://localhost:3000/previews/test-model-preview.png',
            type: 'thumbnail'
          }
        ]
      }

      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResult
      })

      const result = await client.getGenerationResult('test-model-id')

      expect(result).toEqual(mockResult)
      expect(fetch).toHaveBeenCalledWith(
        `${mockBaseUrl}/image-to-3d/result/test-model-id`
      )
    })

    it('should throw error when getting result fails', async () => {
      ;(fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: 'Not Found'
      })

      await expect(
        client.getGenerationResult('non-existent-model-id')
      ).rejects.toThrow('Failed to get generation result: 404 Not Found')
    })
  })
})