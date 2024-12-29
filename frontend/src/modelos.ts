export interface WebApp {
  podcast: Podcast;
  descargas: TotalDescargas;
  descargando: boolean;
  numeroDescargas: number;
}

interface Podcast {
  error_critico?: string;
  errores: string[];
  info: Info;
  programas: Programas[];
}

interface Info {
  nombre?: string;
  descripcion?: string;
  imagen?: string;
}

interface Programas {
  titulo?: string;
  descripcion?: string;
  url?: string;
  descarga?: string;
}

type TotalDescargas = Record<string, InfoDescarga>;

export interface InfoDescarga {
  identificador: string;
  nombre: string;
  archivo: string;
  descarga: string;
  progreso: string;
  finalizado: boolean;
}
