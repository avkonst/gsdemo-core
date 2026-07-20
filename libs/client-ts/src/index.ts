export interface SampleRow {
  id: number;
  name: string;
  value: string;
}

export class CoreClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
  }

  async getRow(id: number, token: string): Promise<SampleRow> {
    const res = await fetch(`${this.baseUrl}/rows/${id}`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error(`Core API error: ${res.status}`);
    return res.json() as Promise<SampleRow>;
  }
}
