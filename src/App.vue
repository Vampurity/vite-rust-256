<script setup>
import { ref, onMounted } from 'vue'
import init, { encrypt_string, decrypt_string, generate_key } from '../pkg/vue_rust_crypto.js'

const inputText = ref('')
const key = ref('')
const encryptedText = ref('')
const encryptedInput = ref('') // 新增：用于手动输入加密内容的输入框
const decryptedText = ref('')
const error = ref('')
const isLoaded = ref(false)

onMounted(async () => {
  try {
    // 加载WebAssembly模块
    await init()
    isLoaded.value = true
    console.log('WebAssembly模块加载成功!')
  } catch (err) {
    error.value = 'WebAssembly模块加载失败: ' + err.message
    console.error('WebAssembly模块加载失败:', err)
  }
})

const generateRandomKey = () => {
  if (isLoaded.value) {
    key.value = generate_key(16)
  }
}

const encrypt = () => {
  if (!isLoaded.value) {
    error.value = '⚠️ WebAssembly模块还未加载'
    return
  }

  if (!inputText.value.trim()) {
    error.value = '✍️ 请输入要加密的文本'
    return
  }

  if (!key.value.trim()) {
    error.value = '🔑 请输入密钥'
    return
  }

  try {
    error.value = ''
    encryptedText.value = encrypt_string(inputText.value, key.value)
    console.log('🔐 加密成功')
  } catch (err) {
    error.value = '加密失败: ' + err.message
    console.error('Encryption error:', err)
  }
}

const decrypt = () => {
  if (!isLoaded.value) {
    error.value = '⚠️ WebAssembly模块还未加载'
    return
  }

  // 优先使用手动输入的加密内容，如果没有则使用加密结果
  const textToDecrypt = encryptedInput.value.trim() || encryptedText.value.trim()

  if (!textToDecrypt) {
    error.value = 'Please enter encrypted text to decrypt'
    return
  }

  if (!key.value.trim()) {
    error.value = 'Please enter a key'
    return
  }

  try {
    error.value = ''
    const result = decrypt_string(textToDecrypt, key.value)
    decryptedText.value = result
    console.log('Decryption successful')
  } catch (err) {
    error.value = 'Decryption failed: ' + err.message
    console.error('Decryption error:', err)
  }
}

const clearAll = () => {
  inputText.value = ''
  key.value = ''
  encryptedText.value = ''
  encryptedInput.value = ''
  decryptedText.value = ''
  error.value = ''
}
</script>

<template>
  <div class="container">
    <header class="header">
      <h1>🔐 Rust WebAssembly 字符串加密解密工具</h1>
      <p class="subtitle">使用 Rust + WebAssembly 实现的高性能加密解密</p>
      <div v-if="!isLoaded" class="loading">
        <div class="spinner"></div>
        <span>正在加载 WebAssembly 模块...</span>
      </div>
    </header>

    <main class="main">
      <div v-if="error" class="error">
        {{ error }}
      </div>

      <div class="form-section">
        <div class="input-group">
          <label for="key">密钥 (Key):</label>
          <div class="key-input">
            <input
              id="key"
              v-model="key"
              type="text"
              placeholder="输入加密密钥"
              :disabled="!isLoaded"
            />
            <button @click="generateRandomKey" :disabled="!isLoaded" class="generate-btn">
              生成随机密钥
            </button>
          </div>
        </div>

        <div class="input-group">
          <label for="input-text">原始文本:</label>
          <textarea
            id="input-text"
            v-model="inputText"
            placeholder="输入要加密的文本"
            :disabled="!isLoaded"
            rows="4"
          ></textarea>
        </div>

        <div class="button-group">
          <button @click="encrypt" :disabled="!isLoaded" class="encrypt-btn">🔒 加密</button>
          <button @click="clearAll" :disabled="!isLoaded" class="clear-btn">🗑️ 清空</button>
        </div>

        <div class="input-group">
          <label for="encrypted-text">加密结果:</label>
          <textarea
            id="encrypted-text"
            v-model="encryptedText"
            placeholder="加密后的文本将显示在这里"
            :disabled="!isLoaded"
            rows="4"
            readonly
          ></textarea>
        </div>

        <div class="input-group">
          <label for="encrypted-input">加密内容输入 (用于解密):</label>
          <textarea
            id="encrypted-input"
            v-model="encryptedInput"
            placeholder="在这里粘贴或输入要解密的加密内容"
            :disabled="!isLoaded"
            rows="4"
          ></textarea>
        </div>

        <div class="button-group">
          <button @click="decrypt" :disabled="!isLoaded" class="decrypt-btn">🔓 解密</button>
        </div>

        <div class="input-group">
          <label for="decrypted-text">解密结果:</label>
          <textarea
            id="decrypted-text"
            v-model="decryptedText"
            placeholder="解密后的文本将显示在这里"
            :disabled="!isLoaded"
            rows="4"
            readonly
          ></textarea>
        </div>
      </div>

      <div class="info-section">
        <h3>📖 使用说明</h3>
        <ul>
          <li>输入一个密钥（或点击"生成随机密钥"）</li>
          <li>在"原始文本"框中输入要加密的内容</li>
          <li>点击"加密"按钮获得加密结果</li>
          <li>
            解密方式有两种：
            <ul>
              <li>使用加密结果自动解密：直接点击"解密"按钮</li>
              <li>手动输入加密内容：在"加密内容输入"框中粘贴加密内容，然后点击"解密"按钮</li>
            </ul>
          </li>
          <li>加密算法使用 XOR 加密，结果以十六进制显示</li>
        </ul>
      </div>
    </main>
  </div>
</template>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.header {
  text-align: center;
  margin-bottom: 40px;
}

.header h1 {
  color: #2c3e50;
  margin-bottom: 10px;
  font-size: 2.5rem;
}

.subtitle {
  color: #7f8c8d;
  font-size: 1.1rem;
  margin-bottom: 20px;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  color: #3498db;
  font-weight: 500;
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid #f3f3f3;
  border-top: 2px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.main {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  padding: 30px;
}

.error {
  background: #fee;
  color: #c33;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 20px;
  border-left: 4px solid #c33;
}

.form-section {
  margin-bottom: 30px;
}

.input-group {
  margin-bottom: 20px;
}

.input-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
  color: #2c3e50;
}

.input-group input,
.input-group textarea {
  width: 100%;
  padding: 12px;
  border: 2px solid #e1e8ed;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.3s ease;
  box-sizing: border-box;
}

.input-group input:focus,
.input-group textarea:focus {
  outline: none;
  border-color: #3498db;
}

.input-group textarea {
  resize: vertical;
  min-height: 80px;
}

.key-input {
  display: flex;
  gap: 10px;
}

.key-input input {
  flex: 1;
}

.generate-btn {
  background: #27ae60;
  color: white;
  border: none;
  padding: 12px 20px;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 500;
  transition: background-color 0.3s ease;
  white-space: nowrap;
}

.generate-btn:hover:not(:disabled) {
  background: #229954;
}

.generate-btn:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
}

.button-group {
  display: flex;
  gap: 15px;
  margin: 20px 0;
}

.encrypt-btn,
.decrypt-btn,
.clear-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
  transition: all 0.3s ease;
}

.encrypt-btn {
  background: #3498db;
  color: white;
}

.encrypt-btn:hover:not(:disabled) {
  background: #2980b9;
  transform: translateY(-1px);
}

.decrypt-btn {
  background: #e74c3c;
  color: white;
}

.decrypt-btn:hover:not(:disabled) {
  background: #c0392b;
  transform: translateY(-1px);
}

.clear-btn {
  background: #95a5a6;
  color: white;
}

.clear-btn:hover:not(:disabled) {
  background: #7f8c8d;
  transform: translateY(-1px);
}

.encrypt-btn:disabled,
.decrypt-btn:disabled,
.clear-btn:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
  transform: none;
}

.info-section {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  border-left: 4px solid #3498db;
}

.info-section h3 {
  color: #2c3e50;
  margin-bottom: 15px;
}

.info-section ul {
  margin: 0;
  padding-left: 20px;
}

.info-section li {
  margin-bottom: 8px;
  color: #555;
  line-height: 1.5;
}

@media (max-width: 600px) {
  .container {
    padding: 10px;
  }

  .header h1 {
    font-size: 2rem;
  }

  .key-input {
    flex-direction: column;
  }

  .button-group {
    flex-direction: column;
  }
}
</style>
