import { useState } from "react";
import { Coins } from "lucide-react";
import { ThemeToggle } from "./components/ThemeToggle";
import { SearchBar } from "./components/SearchBar";
import { NewsCard } from "./components/NewsCard";
import type { NewsArticle } from "./types";

function App() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [articles, setArticles] = useState<NewsArticle[]>([]);

  const handleSearch = async (query: string) => {
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(
        `http://localhost:8080/news?search=${encodeURIComponent(query)}`,
        { method: "GET" }
      );

      if (!response.ok) throw new Error("Failed to fetch news");

      const data = await response.json();
      setArticles(data);
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (err) {
      setError("Failed to fetch news. Please try again later.");
      setArticles([]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <header className="bg-white dark:bg-gray-800 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center space-x-3">
              <Coins className="w-8 h-8 text-blue-500" />
              <h1 className="text-xl font-bold">Crypto News Aggregator</h1>
            </div>
            <ThemeToggle />
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex flex-col items-center space-y-6">
          <SearchBar onSearch={handleSearch} />

          {isLoading && (
            <div className="w-full text-center py-8">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
            </div>
          )}

          {error && (
            <div className="w-full bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-600 dark:text-red-400">
              {error}
            </div>
          )}

          <div className="w-full grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {articles.map((article, index) => (
              <NewsCard key={index} article={article} />
            ))}
          </div>

          {!isLoading && !error && articles.length === 0 && (
            <div className="text-center text-gray-500 dark:text-gray-400 py-8">
              Search for a cryptocurrency to see related news
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
