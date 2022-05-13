export interface Meta {
  url: string,
  contact: string,
  header: Header,
}

export interface Header {
  name: string,
  description: string,
}

export interface Group {
  name: string,
  services: Service[],
}

export interface Service {
  id: string,
  url: string,
  description: string,
}
