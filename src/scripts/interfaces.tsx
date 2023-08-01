export interface CacheData {
  size: number,
  level: number,
  cache_type: String
}

export interface MemData {
  mem_total: number,
  mem_free: number,
  mem_used: number,
}

export interface CpuData {
  vendor: String,
  name: String,
  freq: number,
  util: number,
  threads: number,
  cores: number,
  temperature: number,
  voltage: number,
  package_power: number,
  per_core_freq: Array<number>,
  cache: Array<CacheData>
}

export interface Msr {
  cpu: CpuData,
  memory: MemData,
}

export interface NvStats {
  spec: NvSpec,
  util: NvUtil,
  management: NvManagement
}

export interface NvSpec {
  name: String,
  memory_bus: number,
  memory: number,
  cores: number,
  arc: String,
  pci: Pci,
  cuda: CudaCapability,
  pci_e_gen: number,
  pci_e_width: number,
}

export interface Pci {
  bus: number,
  bus_id: String,
  device: number,
  domain: number,
  pci_device_id: number,
  pci_sub_system_id?: number,
}

export interface CudaCapability {
  major: number,
  minor: number,
}

export interface NvUtil {
  core_usage: number,
  memory_usage: number,
  temperature: number,
  memory_used: number,
  memory_free: number,
  current_core_clock: number,
  current_memory_clock: number,
  power_usage?: number,
}

export interface NvManagement {
  power_limit?: number,
  target_core_clock?: number,
  target_memory_clock?: number,
  default_core_clock?: number,
  default_memory_clock?: number,
  app_core_clock?: number,
  app_memory_clock?: number,
}