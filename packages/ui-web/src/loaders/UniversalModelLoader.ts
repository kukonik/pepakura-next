import * as THREE from 'three'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader'
import { STLLoader } from 'three/examples/jsm/loaders/STLLoader'
import { PLYLoader } from 'three/examples/jsm/loaders/PLYLoader'

/**
 * Универсальный загрузчик 3D моделей
 * Поддерживает различные форматы файлов
 */
export class UniversalModelLoader {
  /**
   * Загрузка модели из файла
   */
  async load(file: File): Promise<THREE.Object3D> {
    const extension = file.name.split('.').pop()?.toLowerCase()
    
    switch (extension) {
      case 'obj':
        return await this.loadOBJ(file)
      case 'stl':
        return await this.loadSTL(file)
      case 'ply':
        return await this.loadPLY(file)
      default:
        throw new Error(`Неподдерживаемый формат файла: ${extension}`)
    }
  }
  
  /**
   * Загрузка OBJ файла
   */
  private async loadOBJ(file: File): Promise<THREE.Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      
      reader.onload = (event) => {
        try {
          const contents = event.target?.result as string
          const loader = new OBJLoader()
          const object = loader.parse(contents)
          resolve(object)
        } catch (error) {
          reject(new Error(`Ошибка загрузки OBJ файла: ${error instanceof Error ? error.message : 'Неизвестная ошибка'}`))
        }
      }
      
      reader.onerror = () => {
        reject(new Error('Ошибка чтения файла'))
      }
      
      reader.readAsText(file)
    })
  }
  
  /**
   * Загрузка STL файла
   */
  private async loadSTL(file: File): Promise<THREE.Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      
      reader.onload = (event) => {
        try {
          const contents = event.target?.result as ArrayBuffer
          const loader = new STLLoader()
          const geometry = loader.parse(contents)
          
          // Создаем материал и меш
          const material = new THREE.MeshStandardMaterial({ 
            color: 0xaaaaaa,
            flatShading: true
          })
          const mesh = new THREE.Mesh(geometry, material)
          
          resolve(mesh)
        } catch (error) {
          reject(new Error(`Ошибка загрузки STL файла: ${error instanceof Error ? error.message : 'Неизвестная ошибка'}`))
        }
      }
      
      reader.onerror = () => {
        reject(new Error('Ошибка чтения файла'))
      }
      
      reader.readAsArrayBuffer(file)
    })
  }
  
  /**
   * Загрузка PLY файла
   */
  private async loadPLY(file: File): Promise<THREE.Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      
      reader.onload = (event) => {
        try {
          const contents = event.target?.result as ArrayBuffer
          const loader = new PLYLoader()
          const geometry = loader.parse(contents)
          
          // Создаем материал и меш
          const material = new THREE.MeshStandardMaterial({ 
            color: 0xaaaaaa,
            flatShading: true
          })
          const mesh = new THREE.Mesh(geometry, material)
          
          resolve(mesh)
        } catch (error) {
          reject(new Error(`Ошибка загрузки PLY файла: ${error instanceof Error ? error.message : 'Неизвестная ошибка'}`))
        }
      }
      
      reader.onerror = () => {
        reject(new Error('Ошибка чтения файла'))
      }
      
      reader.readAsArrayBuffer(file)
    })
  }
}