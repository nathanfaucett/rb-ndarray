require "spec_helper"

describe NDArray do
  it "has a version number" do
    expect(NDArray::VERSION).not_to be nil
  end

  it "create new NDArray" do
    expect(RubyNDArray::new).not_to be nil
  end

  it "should arange new size for NDArray" do
    a = RubyNDArray::new
    a.arange(12)
    expect(a.length).to be 12
    expect(a.rank).to be 1
  end

  it "should reshape new dim for NDArray" do
    a = RubyNDArray::new
    a.reshape([3, 3])
    expect(a.length).to be 9
    expect(a.rank).to be 2
  end
end
