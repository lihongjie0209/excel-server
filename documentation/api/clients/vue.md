# Vue 客户端

完整的 Vue.js 集成示例（支持 Vue 2 和 Vue 3）。

## Vue 3 Composition API

### 基础组件

```vue
<template>
  <div>
    <button @click="exportExcel" :disabled="loading">
      {{ loading ? '导出中...' : '导出 Excel' }}
    </button>
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const loading = ref(false);
const error = ref(null);

const exportExcel = async () => {
  loading.value = true;
  error.value = null;

  const dsl = {
    sheets: [{
      name: "Sheet1",
      cells: [
        { row: 0, col: 0, value: "Hello", value_type: "string" }
      ]
    }]
  };

  try {
    const response = await fetch('/api/excel/async', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(dsl)
    });

    const result = await response.json();
    
    if (result.success) {
      window.location.href = `/api/excel/download/${result.data.file_id}`;
    } else {
      throw new Error(result.message);
    }
  } catch (err) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.error {
  color: red;
  margin-top: 10px;
}
</style>
```

### Composable (useExcelExport)

```typescript
// composables/useExcelExport.ts
import { ref } from 'vue';

export function useExcelExport(baseUrl = '/api/excel') {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const fileId = ref<string | null>(null);

  const generate = async (dsl: any) => {
    loading.value = true;
    error.value = null;
    fileId.value = null;

    try {
      const response = await fetch(`${baseUrl}/async`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(dsl)
      });

      const result = await response.json();

      if (!result.success) {
        throw new Error(result.message);
      }

      const id = result.data.file_id;
      fileId.value = id;
      return id;
    } catch (err: any) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const download = (id?: string) => {
    window.location.href = `${baseUrl}/download/${id || fileId.value}`;
  };

  const generateAndDownload = async (dsl: any) => {
    const id = await generate(dsl);
    download(id);
  };

  return {
    generate,
    download,
    generateAndDownload,
    loading,
    error,
    fileId
  };
}
```

**使用**:

```vue
<template>
  <button @click="handleExport" :disabled="loading">
    {{ loading ? '导出中...' : '导出 Excel' }}
  </button>
</template>

<script setup>
import { useExcelExport } from '@/composables/useExcelExport';

const { generateAndDownload, loading } = useExcelExport();

const handleExport = () => {
  generateAndDownload({
    sheets: [{
      name: "数据",
      cells: [...]
    }]
  });
};
</script>
```

## Vue 2 Options API

```vue
<template>
  <div>
    <button @click="exportExcel" :disabled="loading">
      {{ loading ? '导出中...' : '导出 Excel' }}
    </button>
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      loading: false,
      error: null
    };
  },
  methods: {
    async exportExcel() {
      this.loading = true;
      this.error = null;

      const dsl = {
        sheets: [{
          name: "Sheet1",
          cells: [
            { row: 0, col: 0, value: "Hello", value_type: "string" }
          ]
        }]
      };

      try {
        const response = await fetch('/api/excel/async', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(dsl)
        });

        const result = await response.json();
        
        if (result.success) {
          window.location.href = `/api/excel/download/${result.data.file_id}`;
        } else {
          throw new Error(result.message);
        }
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    }
  }
};
</script>
```

### Vue 2 Mixin

```javascript
// mixins/excelExport.js
export default {
  data() {
    return {
      excelLoading: false,
      excelError: null
    };
  },
  methods: {
    async generateExcel(dsl) {
      this.excelLoading = true;
      this.excelError = null;

      try {
        const response = await fetch('/api/excel/async', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(dsl)
        });

        const result = await response.json();
        
        if (!result.success) {
          throw new Error(result.message);
        }

        return result.data.file_id;
      } catch (err) {
        this.excelError = err.message;
        throw err;
      } finally {
        this.excelLoading = false;
      }
    },
    
    downloadExcel(fileId) {
      window.location.href = `/api/excel/download/${fileId}`;
    }
  }
};
```

**使用**:

```vue
<script>
import excelExportMixin from '@/mixins/excelExport';

export default {
  mixins: [excelExportMixin],
  methods: {
    async handleExport() {
      const fileId = await this.generateExcel({
        sheets: [...]
      });
      this.downloadExcel(fileId);
    }
  }
};
</script>
```

## 完整示例

### 数据表导出

```vue
<template>
  <div class="user-table">
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>姓名</th>
          <th>邮箱</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users" :key="user.id">
          <td>{{ user.id }}</td>
          <td>{{ user.name }}</td>
          <td>{{ user.email }}</td>
        </tr>
      </tbody>
    </table>
    
    <button @click="exportUsers" :disabled="loading">
      <span v-if="loading">导出中...</span>
      <span v-else>导出 Excel</span>
    </button>
    
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useExcelExport } from '@/composables/useExcelExport';

const props = defineProps({
  users: {
    type: Array,
    required: true
  }
});

const { generateAndDownload, loading, error } = useExcelExport();

const exportUsers = () => {
  const dsl = {
    sheets: [{
      name: "用户列表",
      cells: [
        // 表头
        { 
          row: 0, col: 0, value: "ID", value_type: "string",
          style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
        },
        { 
          row: 0, col: 1, value: "姓名", value_type: "string",
          style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
        },
        { 
          row: 0, col: 2, value: "邮箱", value_type: "string",
          style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
        },
        
        // 数据行
        ...props.users.flatMap((user, index) => [
          { row: index + 1, col: 0, value: user.id, value_type: "number" },
          { row: index + 1, col: 1, value: user.name, value_type: "string" },
          { row: index + 1, col: 2, value: user.email, value_type: "string" }
        ])
      ],
      column_widths: {
        0: 10,
        1: 20,
        2: 30
      }
    }]
  };

  generateAndDownload(dsl);
};
</script>

<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

th {
  background-color: #4472C4;
  color: white;
}

button {
  margin-top: 16px;
  padding: 8px 16px;
  background-color: #4472C4;
  color: white;
  border: none;
  cursor: pointer;
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error {
  color: red;
  margin-top: 8px;
}
</style>
```

## Pinia Store

```typescript
// stores/excel.ts
import { defineStore } from 'pinia';

export const useExcelStore = defineStore('excel', {
  state: () => ({
    loading: false,
    error: null as string | null,
    fileId: null as string | null
  }),

  actions: {
    async generate(dsl: any) {
      this.loading = true;
      this.error = null;

      try {
        const response = await fetch('/api/excel/async', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(dsl)
        });

        const result = await response.json();

        if (!result.success) {
          throw new Error(result.message);
        }

        this.fileId = result.data.file_id;
        return this.fileId;
      } catch (err: any) {
        this.error = err.message;
        throw err;
      } finally {
        this.loading = false;
      }
    },

    download(fileId?: string) {
      window.location.href = `/api/excel/download/${fileId || this.fileId}`;
    },

    async generateAndDownload(dsl: any) {
      const fileId = await this.generate(dsl);
      this.download(fileId);
    }
  }
});
```

**使用**:

```vue
<script setup>
import { useExcelStore } from '@/stores/excel';

const excelStore = useExcelStore();

const handleExport = () => {
  excelStore.generateAndDownload({
    sheets: [...]
  });
};
</script>

<template>
  <button @click="handleExport" :disabled="excelStore.loading">
    {{ excelStore.loading ? '导出中...' : '导出 Excel' }}
  </button>
</template>
```

## 下一步

- [JavaScript 客户端](/api/clients/javascript) - Vanilla JS
- [React 客户端](/api/clients/react) - React.js
- [API 文档](/api/overview) - 所有接口
