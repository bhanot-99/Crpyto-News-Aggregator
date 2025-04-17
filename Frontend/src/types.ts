export interface NewsArticle {
  name: string;
  id: string;
  symbol: string;
  logo: string;
  description: string;
  started_at: string;
  link: string;
}

export interface SearchFilters {
  dateRange: 'day' | 'week' | 'month' | 'year';
  sources: string[];
  sortBy: 'relevance' | 'date';
}

export interface CoinPrice {
  price: number;
  change24h: number;
  timestamp: string;
}