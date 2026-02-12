<template>
  <div>
    <div class="mb-8">
      <h2 class="text-3xl font-bold text-white">Time Log</h2>
      <p class="text-slate-400 mt-1">History of all work time entries</p>
    </div>

    <!-- Summary -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <div class="bg-slate-900 p-6 rounded-xl border border-slate-800 shadow-sm flex flex-col">
        <div class="text-3xl font-bold text-white mb-1">{{ allEntries.length }}</div>
        <div class="text-slate-500 text-sm font-medium uppercase tracking-wide">Total Entries</div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl border border-slate-800 shadow-sm flex flex-col">
        <div class="text-3xl font-bold text-blue-400 mb-1">{{ formatHours(totalMinutes) }}</div>
        <div class="text-slate-500 text-sm font-medium uppercase tracking-wide">Total Hours</div>
      </div>
      <div class="bg-slate-900 p-6 rounded-xl border border-slate-800 shadow-sm flex flex-col">
        <div class="text-3xl font-bold text-amber-400 mb-1">{{ uniqueDays }}</div>
        <div class="text-slate-500 text-sm font-medium uppercase tracking-wide">Active Days</div>
      </div>
    </div>

    <!-- Filter -->
    <div class="mb-8">
      <input
        v-model="search"
        class="w-full bg-slate-900 border border-slate-800 rounded-xl px-4 py-3 text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all placeholder-slate-500 shadow-sm"
        placeholder="Search by task or notes..."
      />
    </div>

    <!-- Entries grouped by month -->
    <div v-if="groupedEntries.length" class="space-y-12">
      <div v-for="monthGroup in groupedEntries" :key="monthGroup.monthKey">
        <!-- Month Header -->
        <div class="flex items-center gap-4 mb-6 pb-4 border-b border-slate-800/60">
          <h3 class="text-2xl font-bold text-slate-200 capitalize">{{ monthGroup.monthLabel }}</h3>
          <span class="bg-blue-900/30 text-blue-400 px-3 py-1 rounded-full text-xs font-bold border border-blue-800/50">
            Total: {{ formatHours(monthGroup.totalMinutes) }}
          </span>
        </div>

        <!-- Days in Month -->
        <div class="space-y-6">
          <div v-for="dayGroup in monthGroup.days" :key="dayGroup.date" class="bg-slate-900 rounded-xl border border-slate-800 shadow-sm overflow-hidden">
            <div class="px-6 py-4 bg-slate-800/50 border-b border-slate-800 flex justify-between items-center">
              <h4 class="font-bold text-slate-300">{{ formatDate(dayGroup.date) }}</h4>
              <span class="bg-slate-800 text-slate-400 px-3 py-1 rounded-full text-xs font-bold border border-slate-700">{{ formatMin(dayGroup.totalMinutes) }}</span>
            </div>
            <div class="divide-y divide-slate-800">
              <div v-for="entry in dayGroup.entries" :key="entry.id" class="px-6 py-4 hover:bg-slate-800/30 transition-colors flex justify-between items-start gap-4">
                <div class="flex-1 min-w-0">
                  <div class="font-bold text-white truncate">{{ entry.task_title }}</div>
                  <div class="text-sm text-slate-500 mt-0.5">{{ entry.notes || '—' }}</div>
                </div>
                <div class="font-mono text-slate-400 font-medium whitespace-nowrap">
                  {{ entry.duration_minutes }} min
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="py-16 text-center">
      <div class="text-4xl mb-4 opacity-10">🕐</div>
      <p class="text-slate-500">No time entries yet. Log time from Tasks page!</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useTaskStore } from '~/stores/taskStore'

const store = useTaskStore()
const search = ref('')

onMounted(() => {
  store.fetchAllEntries()
})

const allEntries = computed(() => {
  let entries = store.allEntries
  if (search.value) {
    const q = search.value.toLowerCase()
    entries = entries.filter(
      (e) =>
        (e.task_title || '').toLowerCase().includes(q) ||
        (e.notes || '').toLowerCase().includes(q)
    )
  }
  return entries
})

const totalMinutes = computed(() => allEntries.value.reduce((sum, e) => sum + e.duration_minutes, 0))

const uniqueDays = computed(() => {
  const days = new Set(allEntries.value.map((e) => e.created_at.slice(0, 10)))
  return days.size
})

const groupedEntries = computed(() => {
  const monthGroups = {}

  for (const entry of allEntries.value) {
    const date = entry.created_at.slice(0, 10)
    const monthKey = entry.created_at.slice(0, 7) // YYYY-MM
    
    // Sort logic helper
    if (!monthGroups[monthKey]) {
      monthGroups[monthKey] = {
        monthKey,
        monthLabel: new Date(entry.created_at).toLocaleString('id-ID', { month: 'long', year: 'numeric' }),
        totalMinutes: 0,
        days: {}
      }
    }

    monthGroups[monthKey].totalMinutes += entry.duration_minutes

    if (!monthGroups[monthKey].days[date]) {
      monthGroups[monthKey].days[date] = { date, entries: [], totalMinutes: 0 }
    }

    monthGroups[monthKey].days[date].entries.push(entry)
    monthGroups[monthKey].days[date].totalMinutes += entry.duration_minutes
  }

  // Convert objects to arrays and sort
  return Object.values(monthGroups)
    .sort((a, b) => b.monthKey.localeCompare(a.monthKey))
    .map(month => ({
      ...month,
      days: Object.values(month.days).sort((a, b) => b.date.localeCompare(a.date))
    }))
})

function formatHours(minutes) {
  if (!minutes) return '0'
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (h === 0) return `${m}m`
  if (m === 0) return `${h}h`
  return `${h}h ${m}m`
}

function formatMin(m) {
  if (!m) return '0m'
  const h = Math.floor(m / 60)
  const mins = m % 60
  if (h === 0) return `${mins}m`
  if (mins === 0) return `${h}h`
  return `${h}h ${mins}m`
}

function formatDate(dateStr) {
  const d = new Date(dateStr)
  return d.toLocaleDateString('id-ID', { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' })
}
</script>
