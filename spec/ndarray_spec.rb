require "spec_helper"

describe NDArray do
  it "has a version number" do
    expect(NDArray::VERSION).not_to be nil
  end

  it "create new NDArray" do
    a = RubyNDArray::new
    a.arange(12)
    expect(a).not_to be nil
  end
end
