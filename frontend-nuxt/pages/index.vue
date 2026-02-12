<template>
  <div>
    <div class="mb-8">
      <h2 class="text-3xl font-bold text-white">Dashboard</h2>
      <p class="text-slate-400 mt-1">Today's activity summary</p>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6 animate-fade-in-up">
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800 flex items-center gap-4 hover:border-slate-700 transition-colors">
        <div>
          <div class="text-2xl font-bold text-white">{{ dashboard?.total_tasks ?? 0 }}</div>
          <div class="text-sm text-slate-500 font-medium">Total Tasks</div>
        </div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800 flex items-center gap-4 hover:border-slate-700 transition-colors">
        <div>
          <div class="text-2xl font-bold text-white">{{ dashboard?.completed_tasks ?? 0 }}</div>
          <div class="text-sm text-slate-500 font-medium">Completed</div>
        </div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800 flex items-center gap-4 hover:border-slate-700 transition-colors">
        <div>
          <div class="text-2xl font-bold text-white">{{ formatHours(dashboard?.total_minutes_today ?? 0) }}</div>
          <div class="text-sm text-slate-500 font-medium">Hours Today</div>
        </div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800 flex items-center gap-4 hover:border-slate-700 transition-colors">
        <div>
          <div class="text-2xl font-bold text-white">{{ formatHours(dashboard?.total_minutes_month ?? 0) }}</div>
          <div class="text-sm text-slate-500 font-medium">Hours Month</div>
        </div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800 flex items-center gap-4 hover:border-slate-700 transition-colors">
        <div>
          <div class="text-2xl font-bold text-white">{{ dashboard?.in_progress_tasks ?? 0 }}</div>
          <div class="text-sm text-slate-500 font-medium">In Progress</div>
        </div>
      </div>
    </div>

    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800">
        <div class="mb-6 flex items-center justify-between">
          <h3 class="font-bold text-slate-200 text-lg">📊 Activity Last 7 Days</h3>
        </div>
        <div class="h-72 relative">
          <Bar v-if="barData" :data="barData" :options="barOptions" />
          <div v-else class="absolute inset-0 flex items-center justify-center text-slate-600">
            <p>Loading chart data...</p>
          </div>
        </div>
      </div>

      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800">
        <div class="mb-6 flex items-center justify-between">
          <h3 class="font-bold text-slate-200 text-lg">🍩 Time per Category</h3>
        </div>
        <div class="h-72 relative flex justify-center">
          <Doughnut v-if="doughnutData" :data="doughnutData" :options="doughnutOptions" />
          <div v-else class="absolute inset-0 flex items-center justify-center text-slate-600">
            <p>No category data yet</p>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
      <!-- Recent Notes -->
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800">
        <div class="mb-4 flex items-center justify-between">
          <h3 class="font-bold text-slate-200 text-lg">📝 Recent Notes</h3>
          <span v-if="dashboard?.recent_entries?.length" class="text-xs text-slate-500">{{ dashboard.recent_entries.length }} entries</span>
        </div>
        <div class="relative">
          <div v-if="dashboard?.recent_entries?.length" class="max-h-[280px] overflow-y-auto pr-1 recent-activity-scroll">
            <div class="space-y-3 pl-2 relative border-l-2 border-slate-800 ml-2">
              <div v-for="entry in dashboard.recent_entries" :key="entry.entry_id" class="relative pl-5">
                <div class="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full bg-slate-800 border-2 border-blue-500"></div>
                <div class="flex justify-between items-center gap-2">
                  <div class="min-w-0 flex-1">
                    <div class="font-medium text-slate-200 text-sm truncate">{{ entry.task_title }}</div>
                    <div class="text-xs text-slate-500 truncate">{{ entry.notes || 'No notes' }}</div>
                  </div>
                  <div class="text-xs font-bold text-slate-400 bg-slate-800 px-2 py-0.5 rounded whitespace-nowrap shrink-0">{{ entry.duration_minutes }}m</div>
                </div>
              </div>
            </div>
          </div>
          <!-- Fade hint at bottom when scrollable -->
          <div v-if="dashboard?.recent_entries?.length > 5" class="absolute bottom-0 left-0 right-0 h-8 bg-gradient-to-t from-slate-900 to-transparent pointer-events-none rounded-b-xl"></div>
          <div v-if="!dashboard?.recent_entries?.length" class="py-12 flex flex-col items-center justify-center text-slate-600 text-center">
            <div class="text-4xl mb-3 opacity-30">📭</div>
            <p>No activity yet. Start a task!</p>
          </div>
        </div>
      </div>

      <!-- Category Stats List -->
      <div class="bg-slate-900 p-6 rounded-xl shadow-sm border border-slate-800">
        <div class="mb-6">
          <h3 class="font-bold text-slate-200 text-lg">📂 Category Details</h3>
        </div>
        <div>
          <div v-if="dashboard?.category_stats?.length">
            <div v-for="cat in dashboard.category_stats" :key="cat.category" class="mb-4 last:mb-0">
              <div class="flex justify-between items-center mb-1.5 text-sm">
                <span class="font-medium text-slate-300">{{ cat.category }}</span>
                <span class="text-slate-500 text-xs">{{ cat.task_count }} tasks · {{ formatHours(cat.total_minutes) }}h</span>
              </div>
              <div class="h-2 w-full bg-slate-800 rounded-full overflow-hidden">
                <div class="h-full bg-blue-500 rounded-full" :style="{ width: getCatWidth(cat.total_minutes) + '%' }"></div>
              </div>
            </div>
          </div>
          <div v-else class="py-12 flex flex-col items-center justify-center text-slate-600 text-center">
            <div class="text-4xl mb-3 opacity-30">📊</div>
            <p>No category data yet</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useTaskStore } from '~/stores/taskStore'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  ArcElement
} from 'chart.js'
import { Bar, Doughnut } from 'vue-chartjs'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, BarElement, ArcElement, Title, Tooltip, Legend)

const store = useTaskStore()

onMounted(() => {
  store.fetchDashboard()
})

const dashboard = computed(() => store.dashboard)

// Bar Chart Data (Daily Minutes)
const barData = computed(() => {
  if (!dashboard.value?.daily_minutes) return null
  
  // Fill last 7 days including empty ones
  const labels = []
  const data = []
  const today = new Date()
  
  for (let i = 6; i >= 0; i--) {
    const d = new Date(today)
    d.setDate(today.getDate() - i)
    const dateStr = d.toISOString().slice(0, 10)
    const dayName = d.toLocaleDateString('id-ID', { weekday: 'short' })
    
    labels.push(dayName)
    const entry = dashboard.value.daily_minutes.find(dm => dm.date === dateStr)
    data.push(entry ? entry.minutes : 0)
  }

  return {
    labels,
    datasets: [{
      label: 'Minutes',
      backgroundColor: '#8b5cf6', // Violet
      borderRadius: 6,
      data
    }]
  }
})

const barOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false }
  },
  scales: {
    y: {
      beginAtZero: true,
      grid: { color: 'rgba(255, 255, 255, 0.05)' }
    },
    x: {
      grid: { display: false }
    }
  }
}

// Doughnut Chart Data (Category)
const doughnutData = computed(() => {
  if (!dashboard.value?.category_stats?.length) return null
  
  return {
    labels: dashboard.value.category_stats.map(c => c.category),
    datasets: [{
      backgroundColor: ['#10b981', '#f59e0b', '#f43f5e', '#8b5cf6', '#3b82f6', '#6366f1'],
      borderColor: 'transparent',
      data: dashboard.value.category_stats.map(c => c.total_minutes)
    }]
  }
})

const doughnutOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: 'right', labels: { color: '#94a3b8' } }
  }
}

function formatHours(minutes) {
  if (!minutes) return '0'
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (h === 0) return `${m}m`
  if (m === 0) return `${h}`
  return `${h}h${m}m`
}

function getCatWidth(minutes) {
  if (!dashboard.value?.category_stats?.length) return 0
  const max = Math.max(...dashboard.value.category_stats.map((c) => c.total_minutes))
  if (max === 0) return 0
  return (minutes / max) * 100
}
</script>

<style scoped>
.recent-activity-scroll::-webkit-scrollbar {
  width: 4px;
}
.recent-activity-scroll::-webkit-scrollbar-track {
  background: transparent;
}
.recent-activity-scroll::-webkit-scrollbar-thumb {
  background: rgba(100, 116, 139, 0.3);
  border-radius: 4px;
}
.recent-activity-scroll::-webkit-scrollbar-thumb:hover {
  background: rgba(100, 116, 139, 0.5);
}
</style>
