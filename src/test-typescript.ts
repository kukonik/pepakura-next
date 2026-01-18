// Тестовый TypeScript файл для проверки компиляции
export interface TestUser {
  id: number
  name: string
  email: string
}

export class TestComponent {
  private user: TestUser

  constructor(user: TestUser) {
    this.user = user
  }

  greet(): string {
    return `Hello, ${this.user.name}!`
  }
}

// Пример использования
const user: TestUser = {
  id: 1,
  name: 'Test User',
  email: 'test@example.com'
}

const component = new TestComponent(user)
console.log(component.greet())
