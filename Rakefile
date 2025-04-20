VERSION = File.read('VERSION').chomp

task default: %w(varset:list)

namespace :varset do
  task :list do
    varsets.each { |varset| puts varset }
  end

  task :features do
    puts %{all = [#{varsets.map(&:to_s).map(&:inspect).join(', ')}]}
    varsets.each { |varset| puts "#{varset} = []" }
  end

  task :'features.rs' do
    varsets.each do |varset|
      puts %{    #[cfg(feature = "#{varset}")]}
      puts %{    #{varset.to_s.inspect},}
    end
  end

  task :modules do
    varsets.each do |varset|
      puts
      puts %{#[cfg(feature = "#{varset}")]}
      puts %{pub mod #{varset};}
      puts %{#[cfg(feature = "#{varset}")]}
      puts %{pub use #{varset}::*;}
    end
  end

  task :readme do
    puts %{Varset | Feature | Reference}
    puts %{------ | ------- | ---------}
    varsets.each do |varset|
      puts %{#{varset} | `#{varset}` | [`use getenv::#{varset}::*;`](https://docs.rs/getenv/latest/getenv/varsets/#{varset}/index.html)}
    end
  end
end

def varsets
  require 'pathname'
  result = Dir['lib/getenv/src/varsets/*.rs'].map do |p|
    Pathname(p).basename.sub_ext('').to_s.to_sym
  end
  result.sort
end
